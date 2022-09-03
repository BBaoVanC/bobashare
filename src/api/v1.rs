use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::{IntoResponse, Response},
};
use hyper::{Body, Request, StatusCode};
use thiserror::Error;

use crate::backend::UploadFile;

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("error parsing multipart form data")]
    FormParseError(#[from] MultipartError),
}

/// Accepts: `multipart/form-data`
///
/// Each form field should be a file to upload. The `name` header is ignored.
async fn upload_post(mut form: Multipart) -> axum::response::Result<impl IntoResponse> {
    let mut files: Vec<UploadFile> = Vec::new();
    while let Some(field) = form.next_field().await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "error parsing multipart form data",
        )
    })? {
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
