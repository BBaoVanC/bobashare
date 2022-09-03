use axum::{response::{IntoResponse, Response}, extract::{Multipart, multipart::MultipartError}};
use hyper::{Request, Body, StatusCode};
use thiserror::Error;

use crate::backend::UploadFile;


#[derive(Debug, Error)]
pub enum UploadError {
    #[error("error parsing multipart form data")]
    FormParseError(#[from] MultipartError),
}
// impl IntoResponse for UploadError {
//     fn into_response(self) -> Response {
//         match self {
//             Self::FormParseError(e) => ("error parsing form", StatusCode::INTERNAL_SERVER_ERROR).into_response()
//         }
//     }
// }

/// # Form fields
/// 
/// - 
async fn upload_post(mut form: Multipart) -> axum::response::Result<impl IntoResponse> {
    let mut files: Vec<UploadFile> = Vec::new();
    while let Some(field) = form.next_field().await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "error parsing multipart form data"))? {
        if field.content_type().is_none() { continue; }
        if field.file_name().is_none() { continue; }
        files.append(UploadFile {
            filename:
        });
    }

    Ok(())
}
