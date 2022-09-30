use std::io::{self, ErrorKind};

use axum::{
    extract::multipart::MultipartError,
    response::{IntoResponse, Response},
    Json,
};
use bobashare::storage::file::CreateUploadError;
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;

pub mod upload;

#[derive(Debug, Error)]
pub enum ApiErrorV1 {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Multipart(#[from] MultipartError),
}
impl IntoResponse for ApiErrorV1 {
    fn into_response(self) -> Response {
        let code = match self {
            ApiErrorV1::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorV1::Multipart(_) => StatusCode::BAD_REQUEST,
        };

        let body = Json(json!({"status": "error", "message": self.to_string()}));
        (code, body).into_response()
    }
}
