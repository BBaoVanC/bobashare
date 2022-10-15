//! Routes to display or download an upload in a browser

use std::sync::Arc;

use anyhow::Context;
use axum::{
    body::StreamBody,
    extract::{Path, State},
    response::IntoResponse,
};
use bobashare::storage::{
    file::{FileBackend, OpenUploadError},
    handle::UploadHandle,
};
use hyper::{header, StatusCode};
use thiserror::Error;
use tokio_util::io::ReaderStream;
use tracing::{event, instrument, Level};

use crate::{api::v1::ApiErrorExt, AppState};

/// Errors when trying to view/download an upload
#[derive(Debug, Error)]
pub enum ViewUploadError {
    #[error("an upload at the specified id was not found")]
    NotFound,

    #[error("internal server error")]
    InternalServer(#[from] anyhow::Error),
}
impl IntoResponse for ViewUploadError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        self.into_response_with_code(code)
    }
}

async fn open_upload<S: AsRef<str>>(
    backend: &FileBackend,
    id: S,
) -> Result<UploadHandle, ViewUploadError> {
    let upload = backend
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
        backend
            .delete_upload(id.as_ref())
            .await
            .context("error deleting expired upload")?;
        return Err(ViewUploadError::NotFound);
    }

    Ok(upload)
}

/// Display an upload as HTML
pub async fn display(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ViewUploadError> {
    let _upload = open_upload(&state.backend, id).await?;

    todo!();
    #[allow(unreachable_code)]
    Ok(())
}

/// Download the raw upload file
#[instrument(skip(state))]
pub async fn raw(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ViewUploadError> {
    let upload = open_upload(&state.backend, id).await?;

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
