//! API to create an upload

use std::io::SeekFrom;

use anyhow::Context;
use axum::{
    body::Body,
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{extract::WithRejection, typed_header::TypedHeaderRejection, TypedHeader};
use bobashare::{generate_randomized_id, storage::file::CreateUploadError};
use chrono::{DateTime, TimeDelta, Utc};
use displaydoc::Display;
use futures_util::TryStreamExt;
use headers::{ContentLength, ContentType};
use hyper::{header, HeaderMap, StatusCode};
use serde::Serialize;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufWriter};
use tracing::{event, instrument, Instrument, Level};

use super::ApiErrorExt;
use crate::{clamp_expiry, str_to_duration, AppState};

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
    /// file is too large ({size} > {max})
    TooLarge { size: u64, max: u64 },

    /// upload was cancelled
    Cancelled(#[source] anyhow::Error),

    /// internal server error
    InternalServer(#[from] anyhow::Error),
}
impl From<TypedHeaderRejection> for UploadError {
    fn from(rej: TypedHeaderRejection) -> Self {
        Self::ParseHeader {
            name: rej.name().to_string(),
            source: rej.into(),
        }
    }
}
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::AlreadyExists => StatusCode::CONFLICT,
            Self::ParseHeader { name: _, source: _ } => StatusCode::BAD_REQUEST,
            Self::TooLarge { size: _, max: _ } => StatusCode::PAYLOAD_TOO_LARGE,
            Self::Cancelled(_) => StatusCode::INTERNAL_SERVER_ERROR, // unused
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        if let Self::Cancelled(_) = self {
            let error = anyhow::Error::new(self);
            event!(
                Level::INFO,
                error = format!("{error:#}"),
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
/// `PUT /api/v1/upload/:filename`
///
/// NOTE: The first URL will use the randomized upload ID as the filename.
///
/// ## Headers
///
/// - `Content-Type` (required) -- mimetype -- the mime type (file format) of
///   the file. Note that it will be ignored if the file is plaintext.
/// - `Bobashare-Expiry` (optional) -- number -- duration until the upload
///   should expire
///   - specify `0` for no expiry
///   - examples (see [`str_to_duration`] for more information):
///     - `1d` -- 1 day
///     - `1h` -- 1 hour
///     - `1m` -- 1 minute
///     - `1s` -- 1 second
///
/// - `Bobashare-Delete-Key` (optional) -- string -- custom key to use for
///   deleting the file later; if not provided, one will be randomly generated
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
#[instrument(skip(state, filename, headers, body), fields(id))]
pub async fn put(
    state: State<&'static AppState>,
    filename: Path<String>,
    WithRejection(TypedHeader(mimetype), _): WithRejection<TypedHeader<ContentType>, UploadError>,
    WithRejection(TypedHeader(content_length), _): WithRejection<
        TypedHeader<ContentLength>,
        UploadError,
    >,
    headers: HeaderMap,
    body: Body,
) -> Result<impl IntoResponse, UploadError> {
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
        });
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
                source: anyhow::Error::new(e).context("error converting to string"),
            })?;

            event!(Level::DEBUG, "`Bobashare-Expiry` header says {}", expiry);

            let expiry = if expiry == "never" {
                None
            } else {
                Some(
                    TimeDelta::from_std(str_to_duration(expiry).map_err(|e| {
                        UploadError::ParseHeader {
                            name: String::from("Bobashare-Expiry"),
                            source: anyhow::Error::new(e).context("error parsing duration string"),
                        }
                    })?)
                    .map_err(|e| UploadError::ParseHeader {
                        name: String::from("Bobashare-Expiry"),
                        source: anyhow::Error::new(e).context("error converting duration"),
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
        upload = format!("{upload:?}"),
        "created upload handle"
    );

    let mut file_writer = BufWriter::new(&mut upload.file);
    event!(Level::DEBUG, "streaming file to disk");
    let stream_file_task = async {
        let mut body = body.into_data_stream();
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
                            .context("error writing to file")?;
                    }
                    None => break,
                },
                Err(e) => {
                    return Err(UploadError::Cancelled(e));
                }
            }
        }
        Ok(())
    };

    let mut shutdown_rx = state.shutdown_tx.subscribe();
    tokio::select! {
        res = stream_file_task => {
            if let Err(e) = res {
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
                return Err(e);
            }
        },
        _ = shutdown_rx.recv() => {
            event!(Level::INFO, "server is shutting down; deleting lock");
            upload.drop_lock().await.context("error deleting lock of cancelled upload")?;
            return Err(UploadError::InternalServer(anyhow::anyhow!("server is shutting down")));
        }
    };

    file_writer
        .flush()
        .await
        .context("error flushing file buffer")?;

    let detect_plaintext_span = tracing::span!(Level::INFO, "detect_plaintext");
    async {
        tracing::event!(Level::INFO, "detecting whether the upload is plaintext");
        let upload = &mut upload;
        if let Err(err) = upload.file.seek(SeekFrom::Start(0)).await {
            tracing::event!(Level::ERROR, ?err, "error seeking to beginning of file");
            return;
        };
        let mut buf = [0; 1024];
        if let Err(err) = upload.file.read(&mut buf).await {
            tracing::event!(Level::ERROR, ?err, "error reading first 1024 bytes of file");
            return;
        };

        // TODO: would be nice to support other text encodings
        if std::str::from_utf8(&buf).is_ok() {
            tracing::event!(Level::INFO, "upload is plaintext");
            upload.metadata.mimetype = mime::TEXT_PLAIN_UTF_8;
        } else {
            tracing::event!(Level::INFO, "upload is not plaintext");
        }
    }
    .instrument(detect_plaintext_span)
    .await;

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
