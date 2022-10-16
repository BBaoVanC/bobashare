use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use axum::{self, Router};
use bobashare::storage::file::FileBackend;
use bobashare_web::{api, views, AppState};
use chrono::Duration;
use clap::Parser;
use config::Config;
use hyper::{Body, Request};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{event, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;

#[derive(Debug, Clone, Parser)]
struct Cli {
    /// Enable more logging output (more v's for even lower levels)
    ///
    /// 0. info if release, otherwise info + debug output from bobashare and
    /// tower_http
    ///
    /// 1. same as 0 on a debug build
    ///
    /// 2. debug for everything
    ///
    /// 3. debug, with trace for bobashare
    ///
    /// If you want different levels, use the `RUST_LOG` environment variable
    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..=3))]
    verbose: u8,
    /// Path to config file
    #[arg(short, long, value_name = "CONFIG_FILE", value_parser = validate_config_path)]
    config: Option<PathBuf>,
}
fn validate_config_path(s: &str) -> Result<PathBuf, String> {
    let path = Path::new(s);
    let ext = path.extension().ok_or("Path doesn't have an extension")?;
    if !ext.eq_ignore_ascii_case("toml") {
        return Err(format!("Extension is `{:?}` (expected `toml`)", ext));
    }

    // we're just doing cli parsing, no need for async yet
    let path_canon = std::fs::canonicalize(path).map_err(|e| e.to_string())?;

    Ok(path_canon)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    #[rustfmt::skip]
    let mut config = Config::builder()
        .set_default("listen_addr", "127.0.0.1:3000").unwrap()
        .set_default("backend_path", "storage/").unwrap()
        .set_default("base_url", "http://localhost:3000/").unwrap()
        .set_default("id_length", 8).unwrap()
        .set_default("default_expiry", "24h").unwrap()
        .set_default("max_expiry", Some("30d")).unwrap();

    if let Some(c) = cli.config {
        config = config.add_source(config::File::new(
            c.to_str().unwrap(),
            config::FileFormat::Toml,
        ));
    }

    let config = config
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .context("error loading config")?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                match cli.verbose {
                    0 => {
                        if cfg!(debug_assertions) {
                            "info,bobashare=debug,tower_http=debug"
                        } else {
                            "info"
                        }
                    }
                    1 => "info,bobashare=debug,tower_http=debug",
                    2 => "debug",
                    3 => "debug,bobashare=trace",
                    i => panic!("cli.verbose == {} (out of range)", i),
                }
                .into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    event!(Level::TRACE, ?config);

    let backend =
        FileBackend::new(PathBuf::from(config.get_string("backend_path").unwrap())).await?;
    let base_url: Url = config
        .get_string("base_url")
        .unwrap()
        .parse()
        .context("error parsing `base_url`")?;
    let raw_url = base_url.join("raw/").unwrap();
    let id_length = config.get_int("id_length").unwrap().try_into().unwrap();
    let default_expiry = Duration::from_std(
        duration_str::parse(&config.get_string("default_expiry").unwrap())
            .context("error parsing `default_expiry`")?,
    )
    .unwrap();
    let max_expiry = match config.get_string("max_expiry").unwrap().as_str() {
        "never" => None,
        exp => Some(duration_str::parse(exp).context("error parsing `max_expiry`")?),
    }
    .map(|d| Duration::from_std(d).unwrap());
    let state = Arc::new(AppState {
        backend,
        base_url,
        raw_url,
        id_length,
        default_expiry,
        max_expiry,
    });

    event!(Level::DEBUG,
        backend = ?state.backend,
        base_url = %state.base_url,
        raw_url = %state.raw_url,
        id_length = %state.id_length,
        default_expiry = %state.default_expiry,
        max_expiry = %state.max_expiry.map_or_else(|| String::from("never"), |e| e.to_string()),
        "generated state from config"
    );

    let app = Router::with_state(Arc::clone(&state))
        .nest("/api", api::router(Arc::clone(&state)))
        .merge(views::router(Arc::clone(&state)))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<Body>| {
                        tracing::span!(
                            Level::INFO,
                            "request",
                            method = %request.method(),
                            uri = ?request.uri()
                        )
                    })
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .into_make_service();

    let listen_addr: SocketAddr = config
        .get_string("listen_addr")
        .unwrap()
        .parse()
        .context("error parsing `listen_addr`")?;
    event!(Level::INFO, "Listening on http://{}", listen_addr);
    axum::Server::bind(&listen_addr).serve(app).await?;

    Ok(())
}
