use std::{cmp::Ordering, sync::Arc};

use axum::{
    extract::{multipart::MultipartError, BodyStream, Multipart},
    response::{IntoResponse, Response, Result},
    Extension, Json,
};
use bobashare::storage::file::CreateUploadError;
use chrono::{DateTime, Duration, Utc};
use futures_util::StreamExt;
use hyper::{header, Body, HeaderMap, Request, StatusCode};
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
    #[error("error creating upload: {0}")]
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
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        // let message = Json(json!({"status": "error", "message": self.message}));
        // (self.code, message).into_response()
        let code = match &self {
            UploadError::AlreadyExists => StatusCode::FORBIDDEN,
            UploadError::CreateUpload(_) => StatusCode::INTERNAL_SERVER_ERROR, /* TODO: match */
            // io_error_more
            // errors
            UploadError::Multipart(_) => StatusCode::BAD_REQUEST,
            UploadError::ParseHeader { name: _, source: _ } => StatusCode::BAD_REQUEST,
            UploadError::MissingMultipartContentType(_)
            | UploadError::MissingMultipartFilename(_) => StatusCode::BAD_REQUEST,
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

/// Create an upload that contains a single file
///
/// POST /api/v1/upload
///
/// # Headers
///
/// - `Bobashare-Expiry` -- number -- amount of seconds until the upload should
///   expire
///
/// # Body
///
/// Contents of the file to upload
///
/// # Success
///
/// - 201 Created
/// - `Content-Location` header contains the upload URL as well
/// - Body is JSON, see [`UploadResponse`]
pub async fn post_single_file(
    state: Extension<Arc<AppState>>,
    // headers: HeaderMap,
    // body: Bytes,
    mut request: Request<BodyStream>, // ) -> Result<Json<UploadResponse>, UploadError> {
) -> Result<impl IntoResponse, UploadError> {
    // let expiry = request
    //     .headers()
    //     .get("Bobashare-Expiry")
    //     .and_then(|e| Some(Duration::seconds(e.to_str().ok()?.parse().ok()?)));

    let expiry = match request.headers().get("Bobashare-Expiry") {
        None => Some(state.default_expiry),
        Some(e) => {
            let expiry = e
                .to_str()
                .map_err(|e| UploadError::ParseHeader {
                    name: String::from("Bobashare-Expiry"),
                    source: e.into(),
                })?
                .parse::<i64>()
                .map_err(|e| UploadError::ParseHeader {
                    name: String::from("Bobashare-Expiry"),
                    source: e.into(),
                })?;

            match expiry.cmp(&0) {
                Ordering::Less => Some(state.default_expiry),
                Ordering::Equal => None,
                Ordering::Greater => Some(Duration::seconds(expiry)),
            }
        }
    };

    let upload = state
        .backend
        .create_upload_random_name(state.url_length, expiry)
        .await?;

    todo!();
    Ok((
        StatusCode::CREATED,
        [(header::CONTENT_LOCATION, upload.metadata.url.clone())],
        Json(UploadResponse {
            url: upload.metadata.url,
            expiry_date: upload.metadata.expiry_date,
        }),
    ))
}

/// Create an upload
///
/// # Headers
///
/// - `Bobashare-Expiry` -- number -- amount of seconds until the upload should
///   expire
///   - specify `0` for no expiry
///
/// # Body `multipart/form-data`
///
/// Should contain one field per file to upload. No other fields should be
/// provided.
pub async fn post(
    state: Extension<Arc<AppState>>,
    headers: HeaderMap,
    mut form: Multipart,
) -> Result<Json<UploadResponse>, UploadError> {
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

    let mut upload = state
        .backend
        .create_upload_random_name(state.url_length, expiry)
        .await?;

    let mut i = 0;
    while let Some(field) = form.next_field().await? {
        i += 1; // starts at 1

        todo!();
    }

    todo!();
    // (StatusCode::CREATED, "Created")
}
