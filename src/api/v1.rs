use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::{IntoResponse, Response},
};
use hyper::{Body, Request, StatusCode};
use thiserror::Error;

use crate::backend::UploadFile;
// use axum::response::Result;

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("error parsing multipart form data")]
    FormParseError(#[from] MultipartError),
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        match self {
            UploadError::FormParseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("error parsing form data: {}", e),
            ),
        }
        .into_response()
    }
}

/// Accepts: `multipart/form-data`
///
/// Each form field should be a file to upload. The `name` header is ignored.
async fn upload_post(mut form: Multipart) -> Result<impl IntoResponse, UploadError> {
    let mut files: Vec<UploadFile> = Vec::new();
    while let Some(field) = form.next_field().await? {
        if field.content_type().is_none() {
            continue;
        }
        if field.file_name().is_none() {
            continue;
        }
        // files.append(UploadFile {
        //     filename:
        // });
    }

    Ok(())
}
