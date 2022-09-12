use std::sync::Arc;

use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::{IntoResponse, Response},
    Extension,
};
use bobashare::backend::storage::file::CreateUploadError;
use chrono::Duration;
use hyper::StatusCode;
use thiserror::Error;

use crate::AppState;

// use axum::response::Result;

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("error parsing multipart form data")]
    FormParseError(#[from] MultipartError),
    #[error("error creating upload")]
    CreateUploadError(#[from] CreateUploadError),
    // #[error("error while creating file for upload")]
    // CreateFileError(#[from] CreateFileError<'_>),
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
pub async fn upload_post(
    state: Extension<Arc<AppState>>,
    mut form: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    let mut upload = state
        .backend
        .create_upload("abc123xyz", Some(Duration::hours(1)))
        .await?;
    let mut i = 0;
    while let Some(mut field) = form.next_field().await? {
        i += 1; // starts at 1
        if field.content_type().is_none() {
            continue;
        }
        if field.file_name().is_none() {
            continue;
        }
        let mimetype = field.content_type().unwrap();
        let filename = field.file_name().unwrap();

        // upload
        //     .add_file_from_stream(&format!("{:0<4}", i), field)
        //     .await?;

        // let name_on_disk = format!("{:0<4}", 5);
        todo!();
    }

    Ok(())
}
