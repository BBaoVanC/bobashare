//! Routes to display or download an upload in a browser

use std::sync::Arc;

use anyhow::Context;
use askama::Template;
use axum::{
    body::StreamBody,
    extract::{Path, State},
    response::IntoResponse,
};
use bobashare::storage::{file::OpenUploadError, handle::UploadHandle};
use chrono::{Duration, Utc};
use displaydoc::Display;
use hyper::{header, StatusCode};
use syntect::html::highlighted_html_for_string;
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio_util::io::ReaderStream;
use tracing::{event, instrument, Level};
use url::Url;

use super::{filters, ErrorResponse, ErrorTemplate, TemplateState};
use crate::AppState;

/// Errors when trying to view/download an upload
#[derive(Debug, Error, Display)]
pub enum ViewUploadError {
    /// an upload at the specified id was not found
    NotFound,

    /// internal server error
    InternalServer(#[from] anyhow::Error),
}
impl From<OpenUploadError> for ViewUploadError {
    fn from(err: OpenUploadError) -> Self {
        match err {
            OpenUploadError::NotFound(_) => Self::NotFound,
            _ => Self::InternalServer(anyhow::Error::new(err).context("error opening upload")),
        }
    }
}

async fn open_upload<S: AsRef<str>>(
    state: &AppState,
    id: S,
) -> Result<UploadHandle, ViewUploadError> {
    let upload = state.backend.open_upload(id.as_ref(), false).await?;

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
    state: TemplateState,
    id: String,
    filename: String,
    expiry: Option<Duration>,
    size: u64,
    contents: DisplayType,
    download_url: Url,
}
#[derive(Debug)]
pub enum DisplayType {
    Text(String),
    Binary,
    TooLarge,
}

/// Maximum file size that will be rendered
const MAX_DISPLAY_SIZE: u64 = 1024 * 1024;

/// Display an upload as HTML
#[instrument(skip(state))]
pub async fn display(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut upload = open_upload(&state, id).await.map_err(|e| match e {
        ViewUploadError::NotFound => ErrorTemplate {
            state: state.0.clone().into(),
            code: StatusCode::NOT_FOUND,
            message: e.to_string(),
        },
        ViewUploadError::InternalServer(_) => ErrorTemplate {
            state: state.0.clone().into(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        },
    })?;
    let size = upload
        .file
        .metadata()
        .await
        .map_err(|e| ErrorTemplate {
            state: state.0.clone().into(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("error reading file size: {}", e),
        })?
        .len();

    let contents = if size > MAX_DISPLAY_SIZE {
        DisplayType::TooLarge
    } else {
        let mimetype = upload.metadata.mimetype;
        match (mimetype.type_(), mimetype.subtype()) {
            (mime::TEXT, _) => {
                let extension = std::path::Path::new(&upload.metadata.filename)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                let syntax = state
                    .syntax_set
                    .find_syntax_by_extension(extension)
                    .unwrap_or_else(|| state.syntax_set.find_syntax_plain_text());
                let mut contents = String::with_capacity(size.try_into().unwrap_or(usize::MAX));
                upload
                    .file
                    .read_to_string(&mut contents)
                    .await
                    .map_err(|e| ErrorTemplate {
                        state: state.0.clone().into(),
                        code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: format!("error reading file contents: {}", e),
                    })?;

                // TODO: CONTINUE HERE
                let contents = highlighted_html_for_string(
                    &contents,
                    &state.syntax_set,
                    syntax,
                    &state.syntax_theme,
                ).map_err(|e| ErrorTemplate {
                    state: state.0.clone().into(),
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("error highlighting file contents: {}", e),
                })?;
                DisplayType::Text(contents)
            }
            (mime::APPLICATION, mime::OCTET_STREAM) | (_, _) => DisplayType::Binary,
        }
    };

    event!(Level::DEBUG, "rendering upload template");
    Ok(DisplayTemplate {
        download_url: state.raw_url.join(&upload.metadata.id).unwrap(),
        id: upload.metadata.id,
        filename: upload.metadata.filename,
        expiry: upload.metadata.expiry_date.map(|e| e - Utc::now()),
        size,
        contents,
        state: state.0.into(),
    })
}

/// Download the raw upload file
#[instrument(skip(state))]
pub async fn raw(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ErrorTemplate> {
    let upload = open_upload(&state, id).await.map_err(|e| ErrorTemplate {
        state: state.0.clone().into(),
        code: StatusCode::INTERNAL_SERVER_ERROR,
        message: e.to_string(),
    })?;

    let size = upload
        .file
        .metadata()
        .await
        .map_err(|e| ErrorTemplate {
            state: state.0.clone().into(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("error reading file size: {}", e),
        })?
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
