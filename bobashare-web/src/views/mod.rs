//! Frontend views (as opposed to REST API)

use std::sync::Arc;

use axum::{
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use hyper::StatusCode;
use tower_http::services::{ServeDir, ServeFile};

use crate::AppState;

pub mod upload;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let handler_404 = ServeFile::new("static/404.html");
    Router::with_state(state)
        .route("/:id", get(upload::display))
        .route("/raw/:id", get(upload::raw))
        .fallback_service(get_service(handler_404.clone()).handle_error(handle_static_error))
        .nest(
            "/static",
            get_service(ServeDir::new("static").not_found_service(handler_404))
                .handle_error(handle_static_error),
        )
}

async fn handle_static_error(err: std::io::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("error serving static file: {}", err),
    )
}
