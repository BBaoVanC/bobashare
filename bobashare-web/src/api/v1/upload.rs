use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{BodyStream, Path, State},
    headers::ContentType,
    response::{IntoResponse, Response},
    Json, TypedHeader,
};
use bobashare::{
    generate_randomized_id,
    storage::{file::CreateUploadError, handle::SerializeMetadataError},
};
use chrono::{DateTime, Duration, Utc};
use futures_util::TryStreamExt;
use hyper::{header, HeaderMap, StatusCode};
use serde::Serialize;
use thiserror::Error;
use tokio::io::{self, AsyncWriteExt};
use tracing::{event, instrument, Level};

use crate::{clamp_expiry, AppState};

/// The JSON API response after uploading a file
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub struct UploadResponse {
    /// url to the upload
    url: String,
    /// direct url to download the raw uploaded file
    direct_url: String,
    /// the size in bytes of the uploaded file
    size: Option<u64>,
    /// the MIME type of the uploaded file
    mimetype: String,
    /// expiration date in RFC 3339 format, null if the upload never expires
    expiry_date: Option<DateTime<Utc>>,
}

/// Errors that could occur during upload
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum UploadError {
    AlreadyExists,
    ParseHeader {
        name: String,
        #[serde(serialize_with = "super::serialize_error")]
        source: anyhow::Error,
    },
    #[serde(serialize_with = "super::serialize_error")]
    InternalServer(anyhow::Error),
}
// impl IntoResponse for UploadError {
//     fn into_response(self) -> Response {
//         // let message = Json(json!({"status": "error", "message": self.message}));
//         // (self.code, message).into_response()
//         // match self {
//         //     Self::AlreadyExists => (StatusCode::CONFLICT, "an upload already exists
//         // with the same ") }
//         (StatusCode::INTERNAL_SERVER_ERROR, "abc").into_response()
//     }
// }
impl From<anyhow::Error> for UploadError {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalServer(err)
    }
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        event!(Level::WARN, "error response");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
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
/// ## Body
///
/// Should contain the contents of the file to upload
///
/// # Response
///
/// ## Success
///
/// - 201 Created
/// - `Location` header containing the URL of the upload
/// - JSON body created from [`UploadResponse`]
#[instrument(skip(state, filename, mimetype, headers, body), fields(id))]
pub async fn put(
    state: State<Arc<AppState>>,
    Path(filename): Path<String>,
    TypedHeader(mimetype): TypedHeader<ContentType>,
    headers: HeaderMap,
    mut body: BodyStream,
) -> Result<impl IntoResponse, UploadError> {
    let id = generate_randomized_id(state.id_length);
    tracing::Span::current().record("id", &id);
    event!(Level::DEBUG, "Generated random ID for upload");

    let expiry = match headers.get("Bobashare-Expiry") {
        // if header not found, use default expiry
        None => {
            event!(
                Level::DEBUG,
                "No `Bobashare-Expiry` header provided, using default"
            );
            Some(state.default_expiry)
        }
        // otherwise, clamp the requested expiry to the max
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

            event!(
                Level::DEBUG,
                "`Bobashare-Expiry` header says {} seconds",
                expiry
            );

            let expiry = if expiry == 0 {
                None
            } else {
                Some(Duration::seconds(expiry.into()))
            };

            clamp_expiry(state.max_expiry, expiry)
        }
    };
    event!(
        Level::DEBUG,
        "Final expiry: {:?}",
        expiry.map(|e| e.to_string())
    );

    let mut upload = state
        .backend
        .create_upload(id, filename, mimetype.into(), None, expiry)
        .await
        .map_err(|e| match e {
            CreateUploadError::AlreadyExists => UploadError::AlreadyExists,
            CreateUploadError::Io(e) => UploadError::InternalServer(e.into()),
        })?;
    event!(Level::TRACE, "Created upload: {:?}", upload);

    while let Some(chunk) = body.try_next().await.context("error reading body")? {
        event!(
            Level::TRACE,
            "Writing chunk of {} bytes to file",
            chunk.len()
        );
        upload.file.write_all(&chunk).await.context("error writing to upload file")?;
    }

    event!(Level::TRACE, "Upload is fully written, now finding size...");
    upload.metadata.size = Some(upload.file.metadata().await.context("error reading file metadata to find size")?.len());
    event!(
        Level::DEBUG,
        "Updated size of upload to be {} bytes",
        upload.metadata.size.unwrap()
    );
    let metadata = upload.flush().await.context("error flushing upload metadata to disk")?;
    event!(Level::DEBUG, "Flushed upload metadata to disk");

    let url = state.base_url.join(&metadata.id).unwrap().to_string();
    event!(Level::INFO, "Uploaded file at {}", url);

    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, metadata.id.clone())],
        Json(UploadResponse {
            url,
            direct_url: state.raw_url.join(&metadata.id).unwrap().to_string(),
            size: metadata.size,
            mimetype: metadata.mimetype.to_string(),
            expiry_date: metadata.expiry_date,
        }),
    ))
}
