use std::sync::Arc;

use axum::{
    body::StreamBody,
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use bobashare::storage::file::OpenUploadError;
use hyper::{header, StatusCode};
use serde::Serialize;
use serde_json::json;
use tokio_util::io::ReaderStream;
use tracing::{event, instrument, Level};

use crate::AppState;

#[derive(Debug, Serialize)]
pub enum ViewUploadError {
    #[serde(serialize_with = "crate::serialize_error")]
    NotFound(anyhow::Error),
    #[serde(serialize_with = "crate::serialize_error")]
    InternalServer(anyhow::Error),
}
impl std::fmt::Display for ViewUploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(e) => write!(
                f,
                "NotFound: an upload at the requested id was not found: {:#}",
                e
            ),
            Self::InternalServer(e) => write!(f, "InternalServer: {:#}", e),
        }
    }
}
impl IntoResponse for ViewUploadError {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
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

pub async fn display() {}

// TODO: BUG: `HEAD` request to this endpoint hangs
#[instrument(ret(Debug), err(Display), skip(state))]
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
            _ => ViewUploadError::InternalServer(e.into()),
        })?;

    let size = upload
        .file
        .metadata()
        .await
        .map_err(|e| {
            ViewUploadError::InternalServer(
                anyhow::Error::new(e).context("error getting upload file metadata to read size"),
            )
        })?
        .len();
    event!(
        Level::DEBUG,
        "Found size of upload file to be {} bytes",
        size
    );

    let stream = ReaderStream::new(upload.file);
    let body = StreamBody::new(stream);

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
