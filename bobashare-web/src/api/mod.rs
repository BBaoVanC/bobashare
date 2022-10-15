//! Public facing REST API for bobashare

use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub mod v1;

/// Routes under `/api/`
///
/// - `/api/v1/`: [`v1`]
/// - `/api/latest/`: [`v1`] (latest API version)
pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(Arc::clone(&state))
        .nest("/v1", v1::router(Arc::clone(&state)))
        .nest("/latest", v1::router(Arc::clone(&state)))
}
