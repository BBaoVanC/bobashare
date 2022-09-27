use std::sync::Arc;

use axum::{extract::{multipart::MultipartError, Multipart}, response::{IntoResponse, Response}, Extension};
use bobashare::storage::file::CreateUploadError;
use chrono::Duration;
use hyper::StatusCode;
use thiserror::Error;

use crate::AppState;

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("error parsing multipart form data")]
    FormParseError(#[from] MultipartError),
    #[error("error creating upload")]
    CreateUploadError(#[from] CreateUploadError),
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        match self {
            UploadError::FormParseError(e) => (
                StatusCode::BAD_REQUEST,
                format!("error parsing form data: {}", e),
            ),
            UploadError::CreateUploadError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error creating upload: {}", e),
            ),
            // TODO: make these more specific by matching io::Error
            // UploadError::AddFileError(e) => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     format!("error adding file to upload: {}", e),
            // ),
        }
        .into_response()
    }
}

/// Accepts: `multipart/form-data`
///
/// Each form field should be a file to upload. The `name` header is ignored.
pub async fn post(
    state: Extension<Arc<AppState>>,
    mut form: Multipart,
) -> impl IntoResponse {
    let mut _upload = state
        .backend
        .create_upload("abc123xyz", Some(Duration::hours(1)))
        .await.map_err(|e| match e {
            CreateUploadError::AlreadyExists => (StatusCode::FORBIDDEN, String::from("already exists")),
            CreateUploadError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        });
    let mut i = 0;
    // while let Some(mut field) = form.next_field().await? {
    while let Some(field) = form.next_field().await? {
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

    (StatusCode::CREATED, "Created")
}
