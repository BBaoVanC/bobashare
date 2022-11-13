//! Version 1 of the bobashare API, hosted at `/api/v1/`

use std::{error::Error as StdError, sync::Arc};

use axum::{
    response::{IntoResponse, Response},
    routing::{delete, get, put},
    Json, Router, extract::rejection::TypedHeaderRejection,
};
use displaydoc::Display;
use hyper::StatusCode;
use serde::Serialize;
use serde_json::json;
use tracing::{event, Level};
use utoipa::{OpenApi, ToSchema};

use crate::AppState;

pub mod delete;
pub mod info;
pub mod upload;

#[derive(OpenApi)]
#[openapi(
    paths(delete::delete, info::info, upload::put),
    components(schemas(
        ApiError,
        delete::DeleteResponse,
        info::InfoResponse,
        upload::UploadResponse,
    ))
)]
pub struct ApiDocV1;

/// an error response from the API
#[derive(Debug, Display, Serialize, ToSchema)]
pub struct ApiError {
    /// error message to be returned
    #[schema(example = "an upload at the specified id was not found")]
    pub message: String,

    /// status code to respond with
    #[serde(skip)]
    pub code: StatusCode,
    /// for logging purposes
    #[serde(skip)]
    pub source: Option<Box<dyn StdError>>,
}
impl From<TypedHeaderRejection> for ApiError {
    fn from(rej: TypedHeaderRejection) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: format!("error parsing `{}` header: {}", rej.name(), rej),
            source: Some(rej.into()),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if self.code.is_server_error() {
            event!(Level::ERROR, status = self.code.as_u16(), error = ?self.source);
        } else if self.code.is_client_error() {
            event!(Level::WARN, status = self.code.as_u16(), error = ?self.source);
        } else {
            event!(Level::INFO, status = self.code.as_u16(), error = ?self.source);
        }

        (self.code, Json(self)).into_response()
    }
}

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
pub trait ApiErrorExt: StdError + Sized + Send + Sync + 'static {
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
impl<T> ApiErrorExt for T where T: StdError + Send + Sync + 'static {}
