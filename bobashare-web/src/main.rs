use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{routing::post, Router};
use bobashare::storage::file::FileBackend;
use chrono::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod api;

pub struct AppState {
    pub backend: FileBackend,
    pub url_length: usize,
    pub default_expiry: Duration,
    pub max_expiry: Duration,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let backend_path = "storage/";
    let state = Arc::new(AppState {
        backend: FileBackend::new(PathBuf::from(backend_path)).await?,
        url_length: 8,
        default_expiry: Duration::hours(24),
        max_expiry: Duration::days(30),
    });

    let app = Router::with_state(state).route("/api/v1/upload", post(api::v1::upload::post));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
