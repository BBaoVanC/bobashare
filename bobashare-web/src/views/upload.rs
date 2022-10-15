use std::sync::Arc;

use anyhow::Context;
use axum::{
    body::StreamBody,
    extract::{Path, State},
    response::IntoResponse,
};
use bobashare::storage::file::OpenUploadError;
use hyper::{header, StatusCode};
use thiserror::Error;
use tokio_util::io::ReaderStream;
use tracing::{event, instrument, Level};

use crate::{api::v1::ApiErrorExt, AppState};

#[derive(Debug, Error)]
pub enum ViewUploadError {
    #[error("an upload at the specified id was not found")]
    NotFound(#[source] anyhow::Error),
    #[error("internal server error")]
    InternalServer(#[from] anyhow::Error),
}
impl IntoResponse for ViewUploadError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        self.into_response_with_code(code)
    }
}

pub async fn display() {}

// TODO: BUG: `HEAD` request to this endpoint hangs
// TODO: delete if expired
#[instrument(skip(state))]
pub async fn raw(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ViewUploadError> {
    let upload = state
        .backend
        .open_upload(id, false)
        .await
        .map_err(|e| match e {
            OpenUploadError::NotFound(e) => ViewUploadError::NotFound(e.into()),
            _ => ViewUploadError::InternalServer(
                anyhow::Error::new(e).context("error opening upload"),
            ),
        })?;

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
