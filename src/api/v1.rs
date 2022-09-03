use axum::{response::{IntoResponse, Response}, extract::{Multipart, multipart::MultipartError}};
use hyper::{Request, Body, StatusCode};
use thiserror::Error;


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

async fn upload_post(mut form: Multipart) -> axum::response::Result<impl IntoResponse> {
    // let mut 
    while let Some(field) = form.next_field().await.map_err(|_| ("error parsing multipart form data", StatusCode::INTERNAL_SERVER_ERROR))? {
        // field.
    }

    Ok(())
}
