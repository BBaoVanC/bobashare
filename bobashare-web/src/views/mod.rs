//! Frontend views (as opposed to REST API)

use std::sync::Arc;

use axum::{routing::get, Router};

use crate::AppState;

pub mod upload;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(state)
        .route("/:id", get(upload::display))
        .route("/raw/:id", get(upload::raw))
}
