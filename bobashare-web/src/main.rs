use std::{
    future::IntoFuture,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use anyhow::Context;
use axum::{self, response::Redirect, routing::get, Router};
use bobashare::storage::file::FileBackend;
use bobashare_web::{
    api, render_markdown_with_syntax_set, static_routes, str_to_duration,
    views::{self, ErrorResponse, ErrorTemplate, TemplateState},
    AppState,
};
use chrono::TimeDelta;
use clap::Parser;
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
use serde::Deserialize;
use std::time::Duration as StdDuration;
use chrono::TimeDelta;

#[derive(Debug, Clone, Parser)]
struct Cli {
    /// Enable more logging output (more v's for even lower levels)
    ///
    /// 0. info if release, otherwise info + debug output from bobashare and
    ///    tower_http
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    #[serde(default = "default_listen_addr")]
    listen_addr: SocketAddr,
    #[serde(default = "default_backend_path")]
    backend_path: PathBuf,
    #[serde(default = "default_cleanup_interval")]
    cleanup_interval: StdDuration,
    #[serde(default = "default_base_url")]
    base_url: Url,
    #[serde(default = "default_id_length")]
    id_length: usize,
    #[serde(default = "default_default_expiry")]
    default_expiry: TimeDelta,
    #[serde(default = "default_max_expiry")]
    max_expiry: Option<TimeDelta>,
    #[serde(default = "default_max_file_size")]
    max_file_size: u64,
    #[serde(default)]
    extra_footer_text: Option<String>,
    #[serde(default)]
    about_page: Option<PathBuf>,
}
fn default_listen_addr() -> SocketAddr { "127.0.0.1:3000".parse().unwrap() }
fn default_backend_path() -> PathBuf { PathBuf::from("storage/") }
fn default_cleanup_interval() -> StdDuration { StdDuration::from_hours(1) }
fn default_base_url() -> Url { Url::parse("http://localhost:3000/") }
fn default_id_length() -> usize { 8 }
fn default_default_expiry() -> Option<TimeDelta> { TimeDelta::hours(24) }
fn default_max_expiry() -> Option<TimeDelta> { Some(TimeDelta::days(30)) }
fn default_max_file_size() -> u64 { 1024 * 1024 * 1024 } // 1 GiB

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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
    let cleanup_interval = str_to_duration(&config.get_string("cleanup_interval").unwrap())
        .context("error parsing `cleanup_interval`")?;
    let base_url: Url = config
        .get_string("base_url")
        .unwrap()
        .parse()
        .context("error parsing `base_url`")?;
    let raw_url = base_url.join("raw/").unwrap();
    let id_length = config.get_int("id_length").unwrap().try_into().unwrap();
    let default_expiry = TimeDelta::from_std(
        str_to_duration(&config.get_string("default_expiry").unwrap())
            .context("error parsing `default_expiry`")?,
    )
    .unwrap();
    let max_expiry = match config.get_string("max_expiry").unwrap().as_str() {
        "never" => None,
        exp => Some(str_to_duration(exp).context("error parsing `max_expiry`")?),
    }
    .map(|d| TimeDelta::from_std(d).unwrap());
    let max_file_size = config.get_int("max_file_size").unwrap().try_into().unwrap();

    let syntax_set = SyntaxSet::load_defaults_newlines();

    let extra_footer_text = config.get("extra_footer_text").unwrap();
    let about_page = config
        .get::<Option<String>>("about_page")
        .unwrap()
        .map(PathBuf::from);
    let about_page_content = if let Some(ref path) = about_page {
        // no reason to use tokio here since there is nothing to run concurrently yet
        event!(
            Level::DEBUG,
            "opening about page source file at {}",
            path.display()
        );
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("error reading about page contents at path {path:?}"))?;
        event!(Level::DEBUG, "rendering about page to HTML");
        render_markdown_with_syntax_set(&source, &syntax_set)
            .context("error rendering about page markdown")?
    } else {
        String::new()
    };

    // leak this because tokio requires all captures to be 'static
    let state = Box::leak(Box::new(AppState {
        backend,
        cleanup_interval,
        base_url,
        raw_url,
        id_length,
        default_expiry,
        max_expiry,
        max_file_size,

        syntax_set,

        extra_footer_text,
        about_page,
        about_page_content,

        shutdown_tx: broadcast::channel(4).0,
    }));

    event!(Level::DEBUG,
        backend = ?state.backend,
        base_url = %state.base_url,
        raw_url = %state.raw_url,
        id_length = %state.id_length,
        default_expiry = %state.default_expiry,
        max_expiry = %state.max_expiry.map_or_else(|| String::from("never"), |e| e.to_string()),
        max_file_size = %state.max_file_size,
        extra_footer_text = ?state.extra_footer_text,
        about_page = ?state.about_page,
        "generated state from config"
    );

    let app = Router::new()
        .nest("/api", api::router())
        .merge(views::router())
        .route(
            "/upload",
            get(|| async { Redirect::permanent(state.base_url.as_str()) }),
        )
        .route(
            "/paste",
            get({
                let paste_url = Box::leak(Box::new(state.base_url.join("paste/").unwrap()));
                || async { Redirect::permanent(paste_url.as_str()) }
            }),
        )
        .nest_service("/static", get(static_routes::handler))
        .fallback(|| async {
            ErrorResponse::from(ErrorTemplate {
                code: StatusCode::NOT_FOUND,
                message: "no route for the requested URL was found".into(),
                state: TemplateState::from(&*state),
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
        .with_state(state)
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
