//! Public facing REST API for bobashare

use axum::Router;
use hyper::StatusCode;

use crate::AppState;

pub mod v1;

/// Routes under `/api/`
///
/// - `/api/v1/`: [`v1`]
/// - `/api/latest/`: [`v1`] (latest API version)
pub fn router() -> Router<&'static AppState> {
    Router::new()
        .nest("/v1", v1::router())
        .nest("/latest", v1::router())
        .fallback(|| async { (StatusCode::NOT_FOUND, "error: api route not found") })
}
