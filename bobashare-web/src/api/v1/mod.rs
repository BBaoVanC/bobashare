//! Version 1 of the bobashare API, hosted at `/api/v1/`

use std::sync::Arc;

use axum::{routing::put, Router, error_handling::HandleErrorLayer, BoxError};
use hyper::StatusCode;
use tower::ServiceBuilder;

use crate::AppState;

pub mod upload;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(state)
        .route("/upload", put(upload::put))
        .route("/upload/:filename", put(upload::put))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| {
                    (StatusCode::INTERNAL_SERVER_ERROR, "abc")
                }))
        )
}
