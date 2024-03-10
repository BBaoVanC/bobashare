use std::{
    future::IntoFuture,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use axum::{self, routing::get, Router};
use bobashare::storage::file::FileBackend;
use bobashare_web::{
    api, static_routes,
    views::{self, ErrorResponse, ErrorTemplate},
    AppState,
};
use chrono::TimeDelta;
use clap::Parser;
use config::Config;
use hyper::{Request, StatusCode};
use syntect::parsing::SyntaxSet;
use tokio::{net::TcpListener, signal, sync::broadcast, time::sleep};
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::{event, Instrument, Level};
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
        return Err(format!("Extension is `{ext:?}` (expected `toml`)"));
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
        .set_default("cleanup_interval", "1h").unwrap()
        .set_default("base_url", "http://localhost:3000/").unwrap()
        .set_default("id_length", 8).unwrap()
        .set_default("default_expiry", "24h").unwrap()
        .set_default("max_expiry", Some("30d")).unwrap()
        .set_default("max_file_size", 1024 * 1024 * 1024).unwrap() // 1 GiB
        .set_default("extra_footer_text", None::<String>).unwrap();

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
                    i => panic!("cli.verbose == {i} (out of range)"),
                }
                .into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    event!(Level::TRACE, ?config);

    let backend =
        FileBackend::new(PathBuf::from(config.get_string("backend_path").unwrap())).await?;
    let cleanup_interval = duration_str::parse(config.get_string("cleanup_interval").unwrap())
        .context("error parsing `cleanup_interval`")?;
    let base_url: Url = config
        .get_string("base_url")
        .unwrap()
        .parse()
        .context("error parsing `base_url`")?;
    let raw_url = base_url.join("raw/").unwrap();
    let id_length = config.get_int("id_length").unwrap().try_into().unwrap();
    let default_expiry = TimeDelta::from_std(
        duration_str::parse(config.get_string("default_expiry").unwrap())
            .context("error parsing `default_expiry`")?,
    )
    .unwrap();
    let max_expiry = match config.get_string("max_expiry").unwrap().as_str() {
        "never" => None,
        exp => Some(duration_str::parse(exp).context("error parsing `max_expiry`")?),
    }
    .map(|d| TimeDelta::from_std(d).unwrap());
    let max_file_size = config.get_int("max_file_size").unwrap().try_into().unwrap();
    let extra_footer_text = config.get("extra_footer_text").unwrap();

    let state = Arc::new(AppState {
        backend,
        cleanup_interval,
        base_url,
        raw_url,
        id_length,
        default_expiry,
        max_expiry,
        max_file_size,

        syntax_set: SyntaxSet::load_defaults_newlines(),

        extra_footer_text,

        shutdown_tx: broadcast::channel(4).0,
    });

    event!(Level::DEBUG,
        backend = ?state.backend,
        base_url = %state.base_url,
        raw_url = %state.raw_url,
        id_length = %state.id_length,
        default_expiry = %state.default_expiry,
        max_expiry = %state.max_expiry.map_or_else(|| String::from("never"), |e| e.to_string()),
        max_file_size = %state.max_file_size,
        extra_footer_text = ?state.extra_footer_text,
        "generated state from config"
    );

    let state2 = state.clone();
    let app = Router::new()
        .nest("/api", api::router())
        .merge(views::router())
        .nest_service("/static", get(static_routes::handler))
        .fallback(|| async {
            ErrorResponse(ErrorTemplate {
                code: StatusCode::NOT_FOUND,
                message: "no route for the requested URL was found".into(),
                state: state2.into(),
            })
        })
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            tracing::span!(
                                Level::INFO,
                                "request",
                                method = %request.method(),
                                uri = ?request.uri(),
                                id = ?request.headers().get("X-Request-ID").unwrap()
                            )
                        })
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .propagate_x_request_id(),
        )
        .with_state(state.clone())
        .into_make_service();

    let listen_addr: SocketAddr = config
        .get_string("listen_addr")
        .unwrap()
        .parse()
        .context("error parsing `listen_addr`")?;
    event!(Level::INFO, "listening on http://{}", listen_addr);
    let server_span = tracing::span!(Level::INFO, "server");
    let listener = TcpListener::bind(listen_addr)
        .await
        .context("error binding to listen_addr")?;
    // https://discord.com/channels/442252698964721669/448238009733742612/1216242726920654859
    let server_shutdown_tx = state.shutdown_tx.clone();
    let server_exec = axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            server_shutdown_tx.subscribe().recv().await.unwrap();
            event!(
                Level::INFO,
                "received shutdown signal, quitting axum server"
            );
        })
        .into_future()
        .instrument(server_span);

    let cleanup_span = tracing::span!(Level::INFO, "bg_cleanup");
    let cleanup_exec = async {
        let mut shutdown_rx = state.shutdown_tx.subscribe();
        loop {
            event!(Level::INFO, "running cleanup");
            tokio::select! {
                r = state.backend.cleanup() => {
                    if r.is_err() {
                        event!(Level::ERROR, ?r, "error during cleanup task");
                    } else {
                        event!(Level::INFO, "cleanup done");
                    }
                },
                _ = shutdown_rx.recv() => {
                    event!(Level::INFO, "received shutdown signal, stopping cleanup");
                    break;
                }
            }

            event!(Level::DEBUG, "sleeping before next cleanup task");
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    event!(Level::INFO, "received shutdown signal");
                    break;
                },
                _ = sleep(state.cleanup_interval) => {}
            }
        }
    }
    .instrument(cleanup_span);

    let shutdown_span = tracing::span!(Level::INFO, "shutdown_handler");
    // needed since the shutdown task might outlive main (supposedly?)
    let state2 = state.clone();
    // use a loop and spawn a task so we can keep receiving signals and sending
    // shutdowns
    tokio::spawn(
        async move {
            loop {
                let ctrl_c = async {
                    signal::ctrl_c()
                        .await
                        .expect("failed to install CTRL+C signal handler");
                };

                #[cfg(unix)]
                let terminate = async {
                    signal::unix::signal(signal::unix::SignalKind::terminate())
                        .expect("failed to install SIGTERM signal handler")
                        .recv()
                        .await;
                };
                #[cfg(not(unix))]
                let terminate = std::future::pending::<()>();

                tokio::select! {
                    _ = ctrl_c => {
                        event!(Level::INFO, "received CTRL+C");
                    }
                    _ = terminate => {
                        event!(Level::INFO, "received SIGTERM");
                    }
                }

                if let Err(e) = state2.shutdown_tx.send(()) {
                    event!(Level::ERROR, ?e, "error sending shutdown signal");
                };
            }
        }
        .instrument(shutdown_span),
    );

    // start everything
    let join_results = tokio::join!(server_exec, cleanup_exec);
    join_results.0.context("error running server")?; // handle error in axum server

    Ok(())
}
