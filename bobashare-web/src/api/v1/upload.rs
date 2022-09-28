use std::{sync::Arc, io};

use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::{IntoResponse, Response, ErrorResponse},
    Extension, Json,
};
use bobashare::storage::file::CreateUploadError;
use chrono::Duration;
use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;

use crate::AppState;
// use axum::response::Result;
use super::Result;

// #[derive(Debug, Error, Serialize)]
// pub enum UploadError {
//     #[error("error while reading multipart form data")]
//     MultipartError(#[from] MultipartError),
//     #[error("error while doing i/o")]
//     IoError(#[from] io::Error)
// }

/// Accepts: `multipart/form-data`
///
/// Each form field should be a file to upload. The `name` header is ignored.
pub async fn post(state: Extension<Arc<AppState>>, mut form: Multipart) -> Result<String> {
    let mut _upload = state
        .backend
        .create_upload("abc123xyz", Some(Duration::hours(1)))
        .await
        .map_err(|e| match e {
            CreateUploadError::AlreadyExists => {
                (StatusCode::FORBIDDEN, String::from("already exists"))
            }
            CreateUploadError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        });
    let mut i = 0;
    // while let Some(mut field) = form.next_field().await? {
    while let Some(field) = form.next_field().await.map_err(|e| e.to_string())? {
        i += 1; // starts at 1
        if field.content_type().is_none() {
            continue;
        }
        if field.file_name().is_none() {
            continue;
        }
        let _mimetype = field.content_type().unwrap();
        let _filename = field.file_name().unwrap();

        // let name_on_disk = format!("{:0<4}", 5);
        println!("{}", i); // TODO: remove this
        todo!();
    }

    todo!();
    // (StatusCode::CREATED, "Created")
}
