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
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tracing::{event, instrument, span, Level};

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
#[derive(Debug, Serialize)]
pub enum UploadError {
    AlreadyExists,
    ParseHeader {
        name: String,
        #[serde(serialize_with = "crate::serialize_error")]
        source: anyhow::Error,
    },
    #[serde(serialize_with = "crate::serialize_error")]
    InternalServer(anyhow::Error),
}
impl From<anyhow::Error> for UploadError {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalServer(err)
    }
}
impl std::fmt::Display for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists => write!(
                f,
                "AlreadyExists: an upload already exists with the same id"
            ),
            Self::ParseHeader { name, source } => {
                write!(
                    f,
                    "ParseHeader: error parsing `{}` header: {:#}",
                    name, source
                )
            }
            Self::InternalServer(e) => write!(f, "InternalServer: {:#}", e),
        }
    }
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::AlreadyExists => StatusCode::CONFLICT,
            Self::ParseHeader { name: _, source: _ } => StatusCode::BAD_REQUEST,
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let json = json!({
            "status": "error",
            "message": format!("{}", self),
            "error": self,
        });

        (code, Json(json)).into_response()
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

    let filename = filename.map(|n| n.0).unwrap_or_else(|| id.clone());
    event!(Level::DEBUG, "Filename is {}", filename);

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
            CreateUploadError::Io(e) => UploadError::InternalServer(
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
        if let Ok(mt) = tree_magic::from_filepath(&upload.file_path).parse() {
            event!(Level::DEBUG, "Guessed mimetype to be {}", mt);
            upload.metadata.mimetype = mt;
        } else {
            event!(
                Level::DEBUG,
                "Error while guessing mimetype; it will not be changed"
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
            filename,
            mimetype: metadata.mimetype.to_string(),
            expiry_date: metadata.expiry_date,
        }),
    ))
}
