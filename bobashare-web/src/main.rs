use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{routing::get, Router};
use bobashare::storage::file::FileBackend;
use bobashare_web::{api, views, AppState};
use chrono::Duration;
use clap::Parser;
use hyper::{Body, Request};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;

#[derive(Debug, Clone, Parser)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count, value_parser = clap::value_parser!(u8).range(0..=3))]
    verbose: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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

    let backend_path = "storage/";

    let base_url: Url = "http://localhost:3000/".parse().unwrap();
    let raw_url = base_url.join("raw/").unwrap();

    let state = Arc::new(AppState {
        backend: FileBackend::new(PathBuf::from(backend_path)).await?,
        base_url,
        raw_url,
        id_length: 8,
        default_expiry: Duration::hours(24),
        // max_expiry: Some(Duration::days(30)),
        max_expiry: None,
    });

    let app = Router::with_state(Arc::clone(&state))
        .route("/:id", get(views::upload::display))
        .route("/raw/:id", get(views::upload::raw))
        .nest("/api", api::router(Arc::clone(&state)))
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{}", addr);
    axum::Server::bind(&addr).serve(app).await?;

    Ok(())
}
