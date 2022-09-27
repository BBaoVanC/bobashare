use std::io::{self, ErrorKind};

use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub mod upload;

pub type Result<T = ApiSuccessV1, E = ApiErrorV1> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct ApiResponseV1(StatusCode, ApiResultV1);
impl IntoResponse for ApiResponseV1 {
    fn into_response(self) -> Response {
        (self.0, self.1.into_response())
    }
}
impl From<io::Error> for ApiResponseV1 {
    fn from(err: io::Error) -> Self {
        let code = match err.kind() {
            // TODO: match many of the `ErrorKind`s to different error responses
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self(code, ApiResultV1::Error(ApiErrorV1 { message: err.to_string() }))
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ApiResultV1 {
    #[serde(rename = "success")]
    Success(ApiSuccessV1),
    #[serde(rename = "error")]
    Error(ApiErrorV1),
}

#[derive(Debug, Serialize)]
pub enum ApiSuccessV1 {
}

#[derive(Debug, Serialize)]
pub struct ApiErrorV1 {
    message: String,
}
