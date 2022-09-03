use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::{IntoResponse, Response},
};
use futures_core::Stream;
use hyper::{Body, Request, StatusCode};
use thiserror::Error;

use crate::backend::{storage::UploadRequestFile, UploadFile};
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
                StatusCode::BAD_REQUEST,
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
    let mut files: Vec<UploadRequestFile> = Vec::new();
    while let Some(mut field) = form.next_field().await? {
        if field.content_type().is_none() {
            continue;
        }
        if field.file_name().is_none() {
            continue;
        }
        let mimetype = field.content_type().unwrap();
        let filename = field.file_name().unwrap();

        let contents = Box::new(field.chunk() as dyn Stream);

        files.push(UploadRequestFile {
            filename,
            mimetype,
            contents,
        });
    }

    Ok(())
}
