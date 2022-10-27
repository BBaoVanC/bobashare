//! API to create an upload

use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{BodyStream, Path, State},
    response::{IntoResponse, Response},
    Json,
};
use bobashare::{generate_randomized_id, storage::file::CreateUploadError};
use chrono::{DateTime, Duration, Utc};
use displaydoc::Display;
use futures_util::TryStreamExt;
use hyper::{header, HeaderMap, StatusCode};
use serde::Serialize;
use thiserror::Error;
use tokio::io::{AsyncWriteExt, BufWriter};
use tracing::{event, instrument, span, Level};

use super::ApiErrorExt;
use crate::{clamp_expiry, AppState};

/// The JSON API response after uploading a file
#[derive(Debug, Clone, Serialize)]
pub struct UploadResponse {
    /// ID of the upload (used in URL)
    pub id: String,
    /// url to the upload
    pub url: String,
    /// direct url to download the raw uploaded file
    pub direct_url: String,
    /// the name of the file
    pub filename: String,
    /// the MIME type of the uploaded file
    pub mimetype: String,
    /// expiration date in RFC 3339 format, null if the upload never expires
    pub expiry_date: Option<DateTime<Utc>>,
    /// key to delete the upload later before it's expired
    pub delete_key: String,
}

/// Errors that could occur during upload
#[derive(Debug, Error, Display)]
pub enum UploadError {
    /// an upload already exists with the same id
    AlreadyExists,
    /// error parsing `{name}` header
    ParseHeader { name: String, source: anyhow::Error },

    /// upload was cancelled
    Cancelled(#[source] anyhow::Error),

    /// internal server error
    InternalServer(#[from] anyhow::Error),
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::AlreadyExists => StatusCode::CONFLICT,
            Self::ParseHeader { name: _, source: _ } => StatusCode::BAD_REQUEST,
            Self::Cancelled(_) => StatusCode::INTERNAL_SERVER_ERROR, // unused
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        if let Self::Cancelled(_) = self {
            let error = anyhow::Error::new(self);
            event!(
                Level::INFO,
                error = format!("{:#}", error),
                "returning empty response to cancelled upload"
            );
            ().into_response()
        } else {
            self.into_response_with_code(code)
        }
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
/// - `Content-Type` -- mimetype (optional) -- the mime type (file format) of
///   the file
/// - `Bobashare-Expiry` -- number -- duration until the upload should expire
///   - specify `0` for no expiry
///   - examples (see [`duration_str`] for more information):
///     - `1d` -- 1 day
///     - `1h` -- 1 hour
///     - `1m` -- 1 minute
///     - `1s` -- 1 second
///
/// [`duration_str`]: https://crates.io/crates/duration_str
///
/// - `Bobashare-Delete-Key` -- string -- custom key to use for deleting the
///   file later, instead of a randomly generated one
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
// TODO (outdated): https://github.com/tokio-rs/tracing/pull/2335
// TODO: tracing needs an `ok` instead of `ret` to log just the Ok and not the Err, but workaround
// can be done to log manually
#[instrument(skip(state, filename, headers, body), fields(id))]
pub async fn put(
    state: State<Arc<AppState>>,
    filename: Option<Path<String>>,
    headers: HeaderMap,
    mut body: BodyStream,
) -> Result<impl IntoResponse, UploadError> {
    let id = generate_randomized_id(state.id_length);
    tracing::Span::current().record("id", &id);
    event!(Level::DEBUG, "generated random ID for upload");

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
        event!(Level::DEBUG, filename = n.0);
        (false, n.0)
    } else {
        event!(
            Level::DEBUG,
            filename = id,
            "extension will be guessed later",
        );
        (true, id.clone())
    };

    let expiry = match headers.get("Bobashare-Expiry") {
        // if header not found, use default expiry
        None => {
            event!(
                Level::DEBUG,
                "`Bobashare-Expiry` header not provided, using default"
            );
            Some(state.default_expiry)
        }
        // otherwise, clamp the requested expiry to the max
        Some(e) => {
            let expiry = e.to_str().map_err(|e| UploadError::ParseHeader {
                name: String::from("Bobashare-Expiry"),
                source: anyhow::Error::new(e).context("error converting to string"),
            })?;

            event!(Level::DEBUG, "`Bobashare-Expiry` header says {}", expiry);

            let expiry = if expiry == "never" {
                None
            } else {
                Some(
                    Duration::from_std(duration_str::parse(expiry).map_err(|e| {
                        UploadError::ParseHeader {
                            name: String::from("Bobashare-Expiry"),
                            source: e.context("error parsing duration string"),
                        }
                    })?)
                    .map_err(|e| UploadError::ParseHeader {
                        name: String::from("Bobashare-Expiry"),
                        source: anyhow::Error::new(e).context("error converting duration"),
                    })?,
                )
            };

            clamp_expiry(state.max_expiry, expiry)
        }
    };
    event!(Level::DEBUG, expiry = %expiry.map_or_else(|| String::from("never"), |e| e.to_string()));

    let delete_key = headers
        .get("Bobashare-Delete-Key")
        .map(|k| {
            k.to_str().map_err(|e| UploadError::ParseHeader {
                name: String::from("Bobashare-Delete-Key"),
                source: anyhow::Error::new(e).context("error converting to string"),
            })
        })
        .transpose()?
        .map(ToString::to_string);
    if delete_key.is_some() {
        event!(Level::DEBUG, delete_key, "custom delete key was provided");
    } else {
        event!(Level::DEBUG, "delete_key will be randomly generated");
    }

    let mut upload = state
        .backend
        .create_upload(&id, &filename, mimetype, expiry, delete_key)
        .await
        .map_err(|e| {
            if let CreateUploadError::AlreadyExists = e {
                UploadError::AlreadyExists
            } else {
                UploadError::InternalServer(
                    anyhow::Error::new(e).context("error while initializing upload"),
                )
            }
        })?;
    event!(
        Level::TRACE,
        upload = format!("{:?}", upload),
        "created upload handle"
    );

    let mut file_writer = BufWriter::new(&mut upload.file);
    loop {
        let chunk = body.try_next().await.context("error reading body");
        match chunk {
            Ok(ch) => match ch {
                Some(c) => {
                    event!(Level::TRACE, "writing chunk of {} bytes to file buffer", c.len());
                    file_writer.write_all(&c).await.context("error writing to file")?;
                }
                None => break,
            },
            Err(e) => {
                event!(Level::INFO, "upload was cancelled; it will be deleted");
                upload
                    .flush()
                    .await
                    .context("error flushing cancelled upload before deletion")?;
                state
                    .backend
                    .delete_upload(id)
                    .await
                    .context("error deleting cancelled upload")?;
                event!(Level::INFO, "upload was deleted successfully");
                return Err(UploadError::Cancelled(e));
            }
        }
    }

    if should_guess_mimetype {
        let span = span!(Level::DEBUG, "guess_mimetype");
        let _enter = span.enter();
        event!(
            Level::DEBUG,
            "guessing mimetype since it was not already provided"
        );
        if let Some(Ok(mt)) =
            tree_magic_mini::from_filepath(&upload.file_path).map(|m| m.parse::<mime::Mime>())
        {
            event!(Level::DEBUG, mimetype = mt.to_string(), "guessed");
            upload.metadata.mimetype = mt;
        } else {
            event!(
                Level::DEBUG,
                "error while guessing mimetype; it will not be changed"
            );
        }
    }

    if filename_needs_extension {
        let span = span!(Level::DEBUG, "guess_extension");
        let _enter = span.enter();
        event!(
            Level::DEBUG,
            "guessing file extension since the filename was not provided in request"
        );
        if let Some(ext) = mime_db::extension(&upload.metadata.mimetype) {
            event!(Level::DEBUG, extension = ext, "guessed");
            upload.metadata.filename += &format!(".{}", ext);
        } else {
            event!(
                Level::DEBUG,
                "no extension could be guessed; will not be added"
            );
        }
    }

    let metadata = upload
        .flush()
        .await
        .context("error flushing upload metadata to disk")?;
    event!(Level::DEBUG, "flushed upload metadata to disk");

    // SAFETY: this shouldn't fail because `metadata.id` should be valid in a URL
    let url = state.base_url.join(&metadata.id).unwrap().to_string();
    let direct_url = state.raw_url.join(&metadata.id).unwrap().to_string();
    event!(
        Level::INFO,
        url,
        filename = metadata.filename,
        mimetype = %metadata.mimetype,
        expiry = %metadata
            .expiry_date
            .map_or_else(|| String::from("never"), |e| e.to_string()),
        "successfully created upload"
    );
    Ok((
        StatusCode::CREATED,
        [
            (header::CONTENT_LOCATION, direct_url.clone()),
            (header::LOCATION, url.clone()),
        ],
        Json(UploadResponse {
            id,
            url,
            direct_url,
            filename: metadata.filename,
            mimetype: metadata.mimetype.to_string(),
            expiry_date: metadata.expiry_date,
            delete_key: metadata.delete_key,
        }),
    ))
}
