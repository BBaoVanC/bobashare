use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{routing::put, Router};
use bobashare::storage::file::FileBackend;
use bobashare_web::{api, AppState};
use chrono::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        root_url: "http://localhost:3000/".parse().unwrap(),
        url_length: 8,
        default_expiry: Duration::hours(24),
        max_expiry: Some(Duration::days(30)),
    });

    let app =
        Router::with_state(state).route("/api/v1/upload/:filename", put(api::v1::upload::put));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
