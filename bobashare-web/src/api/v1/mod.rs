use std::io::{self, ErrorKind};

use axum::{
    response::{IntoResponse, Response, ErrorResponse},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod upload;


#[derive(Debug, Error, Serialize)]
#[serde(tag = "type")]
pub enum ApiErrorV1 {
    IoError(#[from] io::Error)
}
impl IntoResponse for ApiErrorV1 {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

// #[derive(Debug, Serialize)]
// pub enum ApiErrorV1 {
//     InternalServerError(String),
// }

// #[derive(Debug)]
// pub struct ApiResponseV1(StatusCode, ApiResultV1);
// impl IntoResponse for ApiResponseV1 {
//     fn into_response(self) -> Response {
//         (self.0, Json(self.1)).into_response()
//     }
// }
// impl From<io::Error> for ApiResponseV1 {
//     fn from(err: io::Error) -> Self {
//         let code = match err.kind() {
//             // TODO: match many of the `ErrorKind`s to different error responses
//             _ => StatusCode::INTERNAL_SERVER_ERROR,
//         };

//         Self(
//             code,
//             ApiResultV1::Error(ApiErrorV1 {
//                 message: err.to_string(),
//             }),
//         )
//     }
// }

// #[derive(Debug, Serialize)]
// pub struct ApiErrorV1 {
//     message: String,
// }
// impl<T: ToString> From<T> for ApiErrorV1 {
//     fn from(err: T) -> Self {
//         Self {
//             message: err.to_string(),
//         }
//     }
// }
