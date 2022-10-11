//! `/api/v1/upload`

use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{BodyStream, Path, State},
    response::{IntoResponse, Response},
    Json,
};
use bobashare::{generate_randomized_id, storage::file::CreateUploadError};
use chrono::{DateTime, Duration, Utc};
use futures_util::TryStreamExt;
use hyper::{header, HeaderMap, StatusCode};
use serde::Serialize;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tracing::{event, instrument, span, Level};

use super::ApiErrorExt;
use crate::{clamp_expiry, AppState};

/// The JSON API response after uploading a file
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub struct UploadResponse {
    /// url to the upload
    url: String,
    /// direct url to download the raw uploaded file
    direct_url: String,
    /// the name of the file
    filename: String,
    /// the MIME type of the uploaded file
    mimetype: String,
    /// expiration date in RFC 3339 format, null if the upload never expires
    expiry_date: Option<DateTime<Utc>>,
}

/// Errors that could occur during upload
#[derive(Debug, Error)]
pub enum UploadError {
    #[error("an upload already exists with the same id")]
    AlreadyExists,
    #[error("error parsing `{}` header", .name)]
    ParseHeader { name: String, source: anyhow::Error },
    #[error("internal server error")]
    InternalServer(#[from] anyhow::Error),
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::AlreadyExists => StatusCode::CONFLICT,
            Self::ParseHeader { name: _, source: _ } => StatusCode::BAD_REQUEST,
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        self.into_response_with_code(code)
    }
}

/// Create an upload
///
/// # Request
///
/// 1. `PUT /api/v1/upload`
/// 2. `PUT /api/v1/upload/:filename`
///
/// NOTE: The first URL will use the randomized upload ID as the filename.
///
/// ## Headers
///
/// - `Bobashare-Expiry` -- number -- amount of seconds until the upload should
///   expire
///   - specify `0` for no expiry
/// - `Content-Type` -- mimetype (optional) -- the mime type (file format) of
///   the file
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
#[instrument(
    ret(Debug),
    err(Display),
    skip(state, filename, headers, body),
    fields(id)
)]
pub async fn put(
    state: State<Arc<AppState>>,
    filename: Option<Path<String>>,
    // TypedHeader(mimetype): TypedHeader<ContentType>,
    headers: HeaderMap,
    mut body: BodyStream,
) -> Result<impl IntoResponse, UploadError> {
    let id = generate_randomized_id(state.id_length);
    tracing::Span::current().record("id", &id);
    event!(Level::DEBUG, "Generated random ID for upload");

    let (should_guess_mimetype, mimetype) = match headers.get(header::CONTENT_TYPE) {
        Some(v) => (
            false,
            std::str::from_utf8(v.as_bytes())
                .map_err(|e| UploadError::ParseHeader {
                    name: header::CONTENT_TYPE.to_string(),
                    source: anyhow::Error::new(e).context("error converting to string"),
                })?
                .parse()
                .map_err(|e| UploadError::ParseHeader {
                    name: header::CONTENT_TYPE.to_string(),
                    source: anyhow::Error::new(e).context("error converting to mimetype"),
                })?,
        ),
        // we will guess this later, default to application/octet-stream for now
        None => (true, mime::APPLICATION_OCTET_STREAM),
    };

    let (filename_needs_extension, filename) = if let Some(n) = filename {
        event!(Level::DEBUG, "Filename is {:?}", n.0);
        (false, n.0)
    } else {
        event!(
            Level::DEBUG,
            "Filename is {:?}; extension will be guessed later",
            id
        );
        (true, id.clone())
    };

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
        .create_upload(&id, &filename, mimetype, expiry)
        .await
        .map_err(|e| match e {
            CreateUploadError::AlreadyExists => UploadError::AlreadyExists,

            CreateUploadError::CreateDirectory(e)
            | CreateUploadError::CreateMetadataFile(e)
            | CreateUploadError::CreateUploadFile(e) => UploadError::InternalServer(
                anyhow::Error::new(e).context("error while initializing upload"),
            ),
        })?;
    event!(Level::TRACE, "Created upload: {:?}", upload);

    while let Some(chunk) = body.try_next().await.context("error reading body")? {
        event!(
            Level::TRACE,
            "Writing chunk of {} bytes to file",
            chunk.len()
        );
        upload
            .file
            .write_all(&chunk)
            .await
            .context("error writing to upload file")?;
    }

    if should_guess_mimetype {
        let span = span!(Level::DEBUG, "guess_mimetype");
        let _enter = span.enter();
        event!(
            Level::DEBUG,
            "Guessing mimetype since it was not already provided"
        );
        if let Some(Ok(mt)) = tree_magic_mini::from_filepath(&upload.file_path).map(|m| m.parse()) {
            event!(Level::DEBUG, "Guessed mimetype to be {}", mt);
            upload.metadata.mimetype = mt;
        } else {
            event!(
                Level::DEBUG,
                "Error while guessing mimetype; it will not be changed"
            );
        }
    }

    if filename_needs_extension {
        let span = span!(Level::DEBUG, "update_extension");
        let _enter = span.enter();
        event!(
            Level::DEBUG,
            "Adding file extension since the filename was not already provided"
        );
        if let Some(ext) = mime_db::extension(&upload.metadata.mimetype) {
            event!(Level::DEBUG, "Picked extension: .{}", ext);
            upload.metadata.filename += &format!(".{}", ext);
        } else {
            event!(
                Level::DEBUG,
                "No extension could be guessed; no extension will be added"
            );
        }
    }

    let metadata = upload
        .flush()
        .await
        .context("error flushing upload metadata to disk")?;
    event!(Level::DEBUG, "Flushed upload metadata to disk");

    let url = state.base_url.join(&metadata.id).unwrap().to_string();
    let direct_url = state.raw_url.join(&metadata.id).unwrap().to_string();
    Ok((
        StatusCode::CREATED,
        [
            (header::CONTENT_LOCATION, direct_url.clone()),
            (header::LOCATION, url.clone()),
        ],
        Json(UploadResponse {
            url,
            direct_url,
            filename: metadata.filename,
            mimetype: metadata.mimetype.to_string(),
            expiry_date: metadata.expiry_date,
        }),
    ))
}
