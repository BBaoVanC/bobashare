//! Public facing REST API for bobashare

use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod v1;

/// Routes under `/api/`
///
/// - `/api/v1/`: [`v1`]
/// - `/api/latest/`: [`v1`] (latest API version)
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/v1", v1::router())
        .nest("/latest", v1::router())
}
