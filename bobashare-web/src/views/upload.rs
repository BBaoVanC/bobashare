//! Routes to display or download an upload in a browser

use std::sync::Arc;

use anyhow::Context;
use askama::Template;
use axum::{
    body::StreamBody,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use bobashare::storage::{file::OpenUploadError, handle::UploadHandle};
use chrono::{Duration, Utc};
use displaydoc::Display;
use hyper::{header, StatusCode};
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio_util::io::ReaderStream;
use tracing::{event, instrument, Level};

use crate::{
    templates::{filters, ErrorTemplate},
    AppState,
};

/// Errors when trying to view/download an upload
#[derive(Debug, Error, Display)]
pub enum ViewUploadError {
    /// an upload at the specified id was not found
    NotFound,

    /// internal server error
    InternalServer(#[from] anyhow::Error),
}
impl ViewUploadError {
    pub fn into_template_response(self, state: &AppState) -> (StatusCode, ErrorTemplate<'_>) {
        let code = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let title = if let Some(reason) = code.canonical_reason() {
            format!("{} {}", code, reason)
        } else {
            format!("{}", code)
        };
        let message = &self.to_string();
        (code, ErrorTemplate {
            state,
            title: &title,
            message,
        })
    }
}

async fn open_upload<S: AsRef<str>>(
    state: &AppState,
    id: S,
) -> Result<UploadHandle, ViewUploadError> {
    let upload = state
        .backend
        .open_upload(id.as_ref(), false)
        .await
        .map_err(|e| match e {
            OpenUploadError::NotFound(_) => ViewUploadError::NotFound,
            _ => ViewUploadError::InternalServer(
                anyhow::Error::new(e).context("error opening upload"),
            ),
        })?;

    if upload.metadata.is_expired() {
        event!(Level::INFO, "upload is expired; it will be deleted");
        // don't upload.flush() since it's not open for writing -- it will fail
        state
            .backend
            .delete_upload(id.as_ref())
            .await
            .context("error deleting expired upload")?;
        return Err(ViewUploadError::NotFound);
    }

    Ok(upload)
}

#[derive(Template)]
#[template(path = "display.html.jinja")]
pub struct DisplayTemplate {
    state: AppState,
    id: String,
    filename: String,
    expiry: Option<Duration>,
    size: u64,
    contents: DisplayType,
}
#[derive(Debug)]
pub enum DisplayType {
    Text(String),
    Binary(String),
    TooLarge(String),
}

/// Maximum file size that will be rendered
const MAX_DISPLAY_SIZE: u64 = 1024 * 1024;

/// Display an upload as HTML
#[instrument(skip(state))]
pub async fn display(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ViewUploadError> {
    let mut upload = open_upload(&state, id).await?;
    let size = upload
        .file
        .metadata()
        .await
        .context("error reading file size")?
        .len();

    let contents = if size > MAX_DISPLAY_SIZE {
        DisplayType::TooLarge(
            state
                .base_url
                .join(&upload.metadata.id)
                .unwrap()
                .to_string(),
        )
    } else {
        let mimetype = upload.metadata.mimetype;
        match (mimetype.type_(), mimetype.subtype()) {
            (mime::TEXT, _) => {
                let mut contents = String::with_capacity(size.try_into().unwrap_or(usize::MAX));
                upload
                    .file
                    .read_to_string(&mut contents)
                    .await
                    .context("error reading file contents")?;
                DisplayType::Text(contents)
            }
            (mime::APPLICATION, mime::OCTET_STREAM) | (_, _) => DisplayType::Binary(
                state
                    .base_url
                    .join(&upload.metadata.id)
                    .unwrap()
                    .to_string(),
            ),
        }
    };
    // let contents = match (upload.metadata.mimetype.type_(),
    // upload.metadata.mimetype.subtype()) {     (mime::TEXT, _) => {

    //     },
    // };

    event!(Level::DEBUG, "rendering upload template");
    Ok(DisplayTemplate {
        state: *state,
        id: upload.metadata.id,
        filename: upload.metadata.filename,
        expiry: upload.metadata.expiry_date.map(|e| e - Utc::now()),
        size,
        contents,
    })
}

/// Download the raw upload file
#[instrument(skip(state))]
pub async fn raw(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ViewUploadError> {
    let upload = open_upload(&state, id).await?;

    let size = upload
        .file
        .metadata()
        .await
        .context("error getting upload file metadata in order to read size")?
        .len();
    event!(Level::DEBUG, size, "found size of upload file",);

    let stream = ReaderStream::new(upload.file);
    let body = StreamBody::new(stream);

    event!(
        Level::INFO,
        "type" = %upload.metadata.mimetype,
        length = size,
        filename = upload.metadata.filename,
        "successfully streaming upload file to client"
    );
    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, upload.metadata.mimetype.to_string()),
            (header::CONTENT_LENGTH, size.to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", upload.metadata.filename),
            ),
        ],
        body,
    ))
}
