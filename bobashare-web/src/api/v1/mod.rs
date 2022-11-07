//! Version 1 of the bobashare API, hosted at `/api/v1/`

use std::{error::Error, sync::Arc};

use axum::{
    response::{IntoResponse, Response},
    routing::{delete, get, put},
    Json, Router,
};
use hyper::StatusCode;
use serde_json::json;
use tracing::{event, Level};

use crate::AppState;

pub mod delete;
pub mod info;
pub mod upload;

/// Routes under `/api/v1/`
///
/// - `/api/v1/info/:id`: [`info::info`]
/// - `/api/v1/upload`: [`upload::put`]
/// - `/api/v1/upload/:filename`: [`upload::put`]
/// - `/api/v1/delete/:id`: [`delete::delete`]
pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(state)
        .route("/info/:id", get(info::info))
        .route("/upload/:filename", put(upload::put))
        .route("/delete/:id", delete(delete::delete))
}

/// Method to convert an [`std::error::Error`] into a [`Response`] with a
/// specified [`StatusCode`]
pub trait ApiErrorExt: Error + Sized + Send + Sync + 'static {
    /// Consume the error and convert it to a [`Response`] with the specified
    /// [`StatusCode`]
    fn into_response_with_code(self, code: StatusCode) -> Response {
        let mut error_msg = self.to_string(); // does not include causes

        let mut err_buf = self.source();
        while let Some(e) = err_buf {
            error_msg += &format!(": {}", e);
            err_buf = e.source();
        }

        if code.is_server_error() {
            event!(Level::ERROR, status = code.as_u16(), error = error_msg);
        } else if code.is_client_error() {
            event!(Level::WARN, status = code.as_u16(), error = error_msg);
        } else {
            event!(Level::INFO, status = code.as_u16(), error = error_msg);
        }

        let resp = json!({
            "status": "error",
            "error": serde_error::Error::new(&self),
            "message": error_msg,
        });
        (code, Json(resp)).into_response()
    }
}
impl<T> ApiErrorExt for T where T: Error + Send + Sync + 'static {}
