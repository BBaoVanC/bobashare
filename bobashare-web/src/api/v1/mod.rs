use std::io::{self, ErrorKind};

use hyper::StatusCode;
use serde::{Deserialize, Serialize};

pub mod upload;

pub type Result<T = ApiSuccessV1, E = ApiErrorV1> = std::result::Result<T, E>;

// TODO: better name for struct
pub struct ApiResponse(StatusCode, ApiResponseV1);
impl From<io::Error> for ApiResponse {
    fn from(err: io::Error) -> Self {
        let code = match err.kind() {
            // TODO: match many of the `ErrorKind`s to different error responses
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // Self::Error { code, err: ApiErrorV1 { message: err.to_string() } }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ApiResponseV1 {
    #[serde(rename = "success")]
    Success(ApiSuccessV1),
    #[serde(rename = "error")]
    Error(ApiErrorV1),
}

#[derive(Debug, Serialize)]
pub enum ApiSuccessV1 {}

#[derive(Debug, Serialize)]
pub struct ApiErrorV1 {
    message: String,
}
