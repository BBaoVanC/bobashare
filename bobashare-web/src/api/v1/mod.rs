//! Version 1 of the bobashare API, hosted at `/api/v1/`

use std::sync::Arc;

use axum::{routing::put, Router};

use crate::AppState;

pub mod upload;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(state)
        .route("/upload", put(upload::put))
        .route("/upload/:filename", put(upload::put))
}
