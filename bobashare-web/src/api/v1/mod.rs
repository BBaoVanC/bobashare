use std::io::{self, ErrorKind};

use axum::{
    response::{IntoResponse, Response, ErrorResponse},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize, ser::SerializeMap};
use thiserror::Error;

pub mod upload;


#[derive(Debug)]
// #[serde(tag = "type")]
pub struct ApiErrorV1 {
    message: String
}
impl Serialize for ApiErrorV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("message", &self.message)?;
        map.end()
    }
}
// impl Serialize for ApiErrorV1 {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//                 let mut s = serializer.serialize_map(Some(1))?;
//                 s.serialize_entry("message", match self {
//                     ApiErrorV1::IoError(e) => e.to_string(),
//                 });
//                 s.end()
//     }
// }
// impl IntoResponse for ApiErrorV1 {
//     fn into_response(self) -> Response {
//         let code = match &self {
//             ApiErrorV1::IoError(e) => match e.kind() {
//                 _ => StatusCode::INTERNAL_SERVER_ERROR,
//             }
//         };
//         (code, Json(self)).into_response()
//     }
// }
