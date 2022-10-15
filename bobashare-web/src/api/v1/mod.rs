//! Version 1 of the bobashare API, hosted at `/api/v1/`

use std::{error::Error, sync::Arc};

use axum::{
    response::{IntoResponse, Response},
    routing::put,
    Json, Router,
};
use hyper::StatusCode;
use serde_json::json;
use tracing::{event, Level};

use crate::AppState;

pub mod upload;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(state)
        .route("/upload", put(upload::put))
        .route("/upload/:filename", put(upload::put))
}

pub trait ApiErrorExt: Error + Sized + Send + Sync + 'static {
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
