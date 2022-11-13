//! API to create an upload

use std::{error::Error as StdError, sync::Arc};

use anyhow::Context;
use axum::{
    extract::{rejection::TypedHeaderRejection, BodyStream, Path, State},
    headers::{ContentLength, ContentType},
    response::IntoResponse,
    Json, TypedHeader,
};
use axum_extra::extract::WithRejection;
use bobashare::{generate_randomized_id, storage::file::CreateUploadError};
use chrono::{DateTime, Duration, Utc};
use displaydoc::Display;
use futures_util::TryStreamExt;
use hyper::{header, HeaderMap, StatusCode};
use serde::Serialize;
use tokio::io::{AsyncWriteExt, BufWriter};
use tracing::{event, instrument, Level};
use utoipa::ToSchema;

use super::ApiError;
use crate::{clamp_expiry, AppState};

/// The JSON API response after uploading a file
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UploadResponse {
    /// ID of the upload (used in URL)
    #[schema(example = "Hk6Shy0Q")]
    pub id: String,
    /// url to the upload
    #[schema(example = "http://localhost:3000/Hk6Shy0Q")]
    pub url: String,
    /// direct url to download the raw uploaded file
    #[schema(example = "http://localhost:3000/raw/Hk6Shy0Q")]
    pub direct_url: String,
    /// the name of the file
    #[schema(example = "example.txt")]
    pub filename: String,
    /// the MIME type of the uploaded file
    #[schema(example = "text/plain")]
    pub mimetype: String,
    /// expiration date in RFC 3339 format, null if the upload never expires
    #[schema(example = "2022-11-13T23:20:09.008416131Z", value_type = String, format = DateTime, nullable)]
    pub expiry_date: Option<DateTime<Utc>>,
    /// key to delete the upload later before it's expired
    #[schema(example = "oM7Yb7N78cAcc7nDOhoo3fHWEl1OphQD")]
    pub delete_key: String,
}

/// Errors that could occur during upload
#[derive(Debug, Display, Serialize)]
pub enum UploadError {
    /// an upload already exists with the same id
    AlreadyExists,
    // /// error parsing `{name}` header
    // ParseHeader {
    //     name: String,
    //     #[serde(skip)]
    //     source: Box<dyn StdError>,
    // },
    /// file is too large ({size} > {max})
    TooLarge { size: u64, max: u64 },

    /// upload was cancelled
    Cancelled(#[serde(skip)] Box<dyn StdError>),

    /// internal server error
    InternalServer(#[serde(skip)] Box<dyn StdError>),
}
impl StdError for UploadError {}
impl From<anyhow::Error> for UploadError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServer(e.into())
    }
}
impl From<UploadError> for ApiError {
    fn from(err: UploadError) -> Self {
        let code = match err {
            UploadError::AlreadyExists => StatusCode::CONFLICT,
            // UploadError::ParseHeader { name: _, source: _ } => StatusCode::BAD_REQUEST,
            UploadError::TooLarge { size: _, max: _ } => StatusCode::PAYLOAD_TOO_LARGE,
            UploadError::Cancelled(_) => StatusCode::from_u16(499).unwrap(), // unused
            UploadError::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            code,
            message: err.to_string(),
            source: Some(err.into()),
        }
    }
}

/// Create an upload
///
/// # Request
///
/// `PUT /api/v1/upload/:filename`
///
/// NOTE: The first URL will use the randomized upload ID as the filename.
///
/// ## Headers
///
/// - `Content-Type` (required) -- mimetype -- the mime type (file format) of
///   the file
/// - `Bobashare-Expiry` (optional) -- number -- duration until the upload
///   should expire
///   - specify `0` for no expiry
///   - examples (see [`duration_str`] for more information):
///     - `1d` -- 1 day
///     - `1h` -- 1 hour
///     - `1m` -- 1 minute
///     - `1s` -- 1 second
///
/// [`duration_str`]: https://crates.io/crates/duration_str
///
/// - `Bobashare-Delete-Key` (optional) -- string -- custom key to use for
///   deleting the file later, instead of a randomly generated one
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
#[utoipa::path(
    context_path = "/api/v1",
    put,
    path = "/upload/{filename}",
    responses(
        (status = 200, description = "TODO")
    )
)]
// #[axum_macros::debug_handler]
pub async fn put(
    state: State<Arc<AppState>>,
    filename: Path<String>,
    WithRejection(TypedHeader(mimetype), _): WithRejection<TypedHeader<ContentType>, ApiError>,
    WithRejection(TypedHeader(content_length), _): WithRejection<
        TypedHeader<ContentLength>,
        ApiError,
    >,
    headers: HeaderMap,
    mut body: BodyStream,
) -> Result<impl IntoResponse, ApiError> {
    // hyper will automatically make sure the body is <= the content-length, so we
    // can rely on it here
    //
    // also note that hyper seems to intercept the Content-Length header and return
    // its own empty response instead of using WithRejection here
    if content_length.0 > state.max_file_size {
        event!(
            Level::INFO,
            size = content_length.0,
            max = state.max_file_size,
            "file is too large"
        );
        return Err(UploadError::TooLarge {
            size: content_length.0,
            max: state.max_file_size,
        }
        .into());
    }
    let id = generate_randomized_id(state.id_length);
    tracing::Span::current().record("id", &id);
    event!(Level::DEBUG, "generated random ID for upload");

    let mimetype = mimetype.into();

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
                source: anyhow::Error::new(e)
                    .context("error converting to string")
                    .into(),
            })?;

            event!(Level::DEBUG, "`Bobashare-Expiry` header says {}", expiry);

            let expiry = if expiry == "never" {
                None
            } else {
                Some(
                    Duration::from_std(duration_str::parse(expiry).map_err(|e| {
                        UploadError::ParseHeader {
                            name: String::from("Bobashare-Expiry"),
                            source: e.context("error parsing duration string").into(),
                        }
                    })?)
                    .map_err(|e| UploadError::ParseHeader {
                        name: String::from("Bobashare-Expiry"),
                        source: anyhow::Error::new(e)
                            .context("error converting duration")
                            .into(),
                    })?,
                )
            };

            // TODO: should we return an error if expiry is too large instead?
            clamp_expiry(state.max_expiry, expiry)
        }
    };
    event!(Level::DEBUG, expiry = %expiry.map_or_else(|| String::from("never"), |e| e.to_string()));

    let delete_key = headers
        .get("Bobashare-Delete-Key")
        .map(|k| {
            k.to_str().map_err(|e| UploadError::ParseHeader {
                name: String::from("Bobashare-Delete-Key"),
                source: anyhow::Error::new(e)
                    .context("error converting to string")
                    .into(),
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
                    anyhow::Error::new(e)
                        .context("error while initializing upload")
                        .into(),
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
                    event!(
                        Level::TRACE,
                        "writing chunk of {} bytes to file buffer",
                        c.len()
                    );
                    file_writer
                        .write_all(&c)
                        .await
                        .context("error writing to file")
                        .map_err(UploadError::from)?;
                }
                None => break,
            },
            Err(e) => {
                event!(Level::INFO, "upload was cancelled; it will be deleted");
                upload
                    .flush()
                    .await
                    .context("error flushing cancelled upload before deletion")
                    .map_err(UploadError::from)?;
                state
                    .backend
                    .delete_upload(id)
                    .await
                    .context("error deleting cancelled upload")
                    .map_err(UploadError::from)?;
                event!(Level::INFO, "upload was deleted successfully");
                return Err(UploadError::Cancelled(e.into()).into());
            }
        }
    }
    file_writer
        .flush()
        .await
        .context("error flushing file buffer")
        .map_err(UploadError::from)?;

    let metadata = upload
        .flush()
        .await
        .context("error flushing upload metadata to disk")
        .map_err(|e| UploadError::InternalServer(e.into()))?;
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
