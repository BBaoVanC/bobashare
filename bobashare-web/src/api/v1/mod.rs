//! Version 1 of the bobashare API, hosted at `/api/v1/`

use std::{error::Error, sync::Arc};

use axum::{
    response::{IntoResponse, Response},
    routing::put,
    Json, Router,
};
use hyper::StatusCode;
use serde_json::json;

use crate::AppState;

pub mod upload;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(state)
        .route("/upload", put(upload::put))
        .route("/upload/:filename", put(upload::put))
}

pub trait ApiErrorExt: Error + Sized + Send + Sync + 'static {
    fn into_response_with_code(self, code: StatusCode) -> Response {
        let resp = json!({
            "status": "error",
            "error": serde_error::Error::new(&self),
            "message": format!("{:#}", anyhow::Error::new(self)),
        });
        (code, Json(resp)).into_response()
    }
}
impl<T> ApiErrorExt for T where T: Error + Send + Sync + 'static {}
