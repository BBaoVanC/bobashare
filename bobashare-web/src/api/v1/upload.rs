use std::sync::Arc;

use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::{IntoResponse, Response, Result},
    Extension, Json,
};
use bobashare::storage::{file::CreateUploadError, handle::SerializeMetadataError};
use chrono::{DateTime, Duration, Utc};
use hyper::{header, HeaderMap, StatusCode};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

use crate::AppState;

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub struct UploadResponse {
    /// url to the upload
    url: String,
    /// expiration date in RFC 3339 format, null if the upload never expires
    expiry_date: Option<DateTime<Utc>>,
    // /// metadata about each of the uploaded files
    // files: HashMap<String, FileResponse>,
}
#[derive(Debug, Serialize)]
pub struct FileResponse {
    /// size of the file in bytes
    size: usize,
    /// url to download the file directly
    url: String,
    /// the MIME type of the file
    mimetype: String,
}

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("an upload at the url already exists")]
    AlreadyExists,
    #[error("internal error creating upload: {0}")]
    CreateUpload(CreateUploadError),
    #[error("failed to parse {name} header: {source}")]
    ParseHeader { name: String, source: anyhow::Error },
    #[error("error parsing multipart form data: {0}")]
    Multipart(#[from] MultipartError),
    #[error("missing Content-Type in the multipart field {0}")]
    MissingMultipartContentType(String),
    #[error("missing Content-Disposition (filename) in the multipart field {0}")]
    MissingMultipartFilename(String),
    #[error("requested expiry is too long: {0}")]
    ExpiryTooLong(Duration),
    #[error("internal error while serializing upload metadata: {0}")]
    SerializeMetadata(#[from] SerializeMetadataError),
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        // let message = Json(json!({"status": "error", "message": self.message}));
        // (self.code, message).into_response()
        let code = match &self {
            UploadError::AlreadyExists => StatusCode::FORBIDDEN,

            // TODO: handle io_error_more errors
            UploadError::CreateUpload(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UploadError::SerializeMetadata(_) => StatusCode::INTERNAL_SERVER_ERROR,

            UploadError::Multipart(_) => StatusCode::BAD_REQUEST,
            UploadError::ParseHeader { name: _, source: _ } => StatusCode::BAD_REQUEST,
            UploadError::MissingMultipartContentType(_) => StatusCode::BAD_REQUEST,
            UploadError::MissingMultipartFilename(_) => StatusCode::BAD_REQUEST,
            UploadError::ExpiryTooLong(_) => StatusCode::BAD_REQUEST,
        };
        let message = Json(json!({
            "status": "error",
            "message": self.to_string()
        }));

        (code, message).into_response()
    }
}
impl From<CreateUploadError> for UploadError {
    fn from(err: CreateUploadError) -> Self {
        match err {
            CreateUploadError::AlreadyExists => Self::AlreadyExists,
            _ => Self::CreateUpload(err),
        }
    }
}

/// Create an upload
///
/// # Request
///
/// ## Headers
///
/// - `Bobashare-Expiry` -- number -- amount of seconds until the upload should
///   expire
///   - specify `0` for no expiry
///
/// ## Body `multipart/form-data`
///
/// Should contain one field per file to upload. No other fields should be
/// provided.
///
/// # Response
///
/// ## Success
///
/// - 201 Created
/// - `Location` header containing the URL of the upload
/// - JSON body created from [`UploadResponse`]
pub async fn post(
    state: Extension<Arc<AppState>>,
    headers: HeaderMap,
    mut form: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    let expiry = match headers.get("Bobashare-Expiry") {
        None => Some(state.default_expiry),
        Some(e) => {
            let expiry = e
                .to_str()
                .map_err(|e| UploadError::ParseHeader {
                    name: String::from("Bobashare-Expiry"),
                    source: e.into(),
                })?
                .parse::<u32>()
                .map_err(|e| UploadError::ParseHeader {
                    name: String::from("Bobashare-Expiry"),
                    source: e.into(),
                })?;

            if expiry == 0 {
                None
            } else {
                let duration = Duration::seconds(expiry.into());
                if duration > state.max_expiry {
                    return Err(UploadError::ExpiryTooLong(duration));
                } else {
                    Some(duration)
                }
            }
        }
    };

    let upload = state
        .backend
        .create_upload_random_name(state.url_length, expiry)
        .await?;

    let mut i = 0;
    while let Some(field) = form.next_field().await? {
        i += 1; // starts at 1

        println!("{}", i);
        todo!();
    }

    let metadata = upload.flush().await?;

    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, metadata.url.clone())],
        Json(UploadResponse {
            url: metadata.url,
            expiry_date: metadata.expiry_date,
        }),
    ))
}
