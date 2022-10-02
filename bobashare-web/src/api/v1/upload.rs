use std::{collections::HashMap, sync::Arc};

use axum::{
    body::Bytes,
    extract::{multipart::MultipartError, Multipart},
    http::HeaderValue,
    response::{IntoResponse, Response, Result},
    Extension, Json, headers,
};
use bobashare::storage::file::CreateUploadError;
use chrono::{DateTime, Duration, Utc};
use hyper::{HeaderMap, StatusCode};
use serde::Serialize;
use serde_json::json;

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

#[derive(Debug)]
pub struct UploadError {
    code: StatusCode,
    message: String,
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let message = Json(json!({"status": "error", "message": self.message}));
        (self.code, message).into_response()
    }
}
impl From<MultipartError> for UploadError {
    fn from(err: MultipartError) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: err.to_string(),
        }
    }
}
impl From<CreateUploadError> for UploadError {
    fn from(err: CreateUploadError) -> Self {
        match err {
            CreateUploadError::AlreadyExists => Self {
                code: StatusCode::FORBIDDEN,
                message: String::from("an upload with the requested name already exists"),
            },
            CreateUploadError::Io(e) => {
                // See https://github.com/rust-lang/rust-clippy/issues/9575
                #[allow(clippy::match_single_binding)]
                let code = match e.kind() {
                    // TODO: track io_error_more and once it's added, match the relevant errors
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                Self {
                    code,
                    message: e.to_string(),
                }
            }
        }
    }
}

/// PUT /api/v1/upload
///
/// # Headers
///
/// - `Bobashare-Expiry` -- number -- amount of seconds until the upload should
///   expire
///
/// # Body
///
/// - contents of the single file to upload
/// 
/// # Success
/// 
/// - 201 Created
/// - Body is JSON, see [`UploadResponse`]
///
/// # Description
///
/// This will create an upload that contains a single file.
pub async fn put(
    state: Extension<Arc<AppState>>,
    // headers: HeaderMap,
    // HeaderValue(expiry_header): HeaderValue,
    body: Bytes,
// ) -> Result<Json<UploadResponse>, UploadError> {
) -> Result<impl IntoResponse, UploadError> {
    // TODO: get expiry from header
    // let expiry = headers.get("Bobashare-Expiry").map(|e| Duration::seconds(e));
    let upload = state
        .backend
        .create_upload_random_name(state.url_length, None)
        .await?;

    Ok(([headers::ContentLocation],
        Json(UploadResponse {
        url: upload.url,
        expiry_date: upload.expiry_date,
    })))
}

/// Accepts: `multipart/form-data`
///
/// Each form field should be a file to upload. The `name` header is ignored.
pub async fn post(
    state: Extension<Arc<AppState>>,
    mut form: Multipart,
) -> Result<Json<UploadResponse>, UploadError> {
    // need function to set duration after the fact
    let mut name: Option<String> = None;
    let mut duration: Option<Duration> = None;
    // let mut files = Vec::new();

    // while let Some(field) = form.next_field().await? {
    //     match field.name().ok_or(UploadInternalError {
    //         code: StatusCode::BAD_REQUEST,
    //         message: String::from("received a multipart field with no name"),
    //     })? {

    //     }
    // }

    let mut upload = state
        .backend
        .create_upload("abc123xyz", Some(Duration::hours(1)))
        .await?;
    // .await
    // .map_err(|e| match e {
    //     CreateUploadError::AlreadyExists => {
    //         (StatusCode::FORBIDDEN, String::from("already exists"))
    //     }
    //     CreateUploadError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR,
    // e.to_string()), });
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

    todo!();
    // (StatusCode::CREATED, "Created")
}
