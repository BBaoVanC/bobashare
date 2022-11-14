//! Version 1 of the bobashare API, hosted at `/api/v1/`

use std::{error::Error as StdError, sync::Arc};

use axum::{
    extract::rejection::TypedHeaderRejection,
    response::{IntoResponse, Response},
    routing::{delete, get, put},
    Json, Router,
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
// #[derive(Debug, Display, SerializeDisplay, ToSchema)]
// #[serde(tag = "error", content = "message")]
#[non_exhaustive]
pub enum ApiError {
    /// an upload at the specified id was not found
    NotFound,
    /// error parsing `{name}` header: {source}
    InvalidHeader {
        /// name of the header
        name: String,
        #[serde(skip)]
        source: Box<dyn StdError + Send + Sync>,
    },
    /// incorrect delete key
    IncorrectKey,
    /// file is too large ({size} > {max})
    TooLarge {
        /// size of the file
        size: u64,
        /// maximum size of the file
        max: u64,
    },
    /// the upload was cancelled
    Cancelled,
    /// an upload at the specified id already exists
    AlreadyExists,

    /// internal server error: {source}
    InternalServer {
        #[serde(skip)]
        source: Box<dyn StdError + Send + Sync>,
    },
}
impl From<TypedHeaderRejection> for ApiError {
    fn from(rej: TypedHeaderRejection) -> Self {
        Self::InvalidHeader {
            name: rej.name().to_string(),
            source: rej.into(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InvalidHeader { .. } => StatusCode::BAD_REQUEST,
            Self::IncorrectKey => StatusCode::FORBIDDEN,
            Self::TooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Cancelled => StatusCode::from_u16(499).unwrap(),
            Self::AlreadyExists => StatusCode::CONFLICT,

            Self::InternalServer { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        };

        if code.is_server_error() {
            event!(Level::ERROR, status = code.as_u16(), error = ?self);
        } else if code.is_client_error() {
            event!(Level::WARN, status = code.as_u16(), error = ?self);
        } else {
            event!(Level::INFO, status = code.as_u16(), error = ?self);
        }

        (
            code,
            Json(json!({"error": self, "message": self.to_string()})),
        )
            .into_response()
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
