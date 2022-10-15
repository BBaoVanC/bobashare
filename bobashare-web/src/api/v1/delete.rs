//! API to delete an upload

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use bobashare::storage::file::{DeleteUploadError, OpenUploadError};
use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;
use tracing::{event, instrument, Level};

use super::ApiErrorExt;
use crate::AppState;

/// API response after deleting an upload successfully
#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    /// The ID of the deleted upload
    pub id: String,
}

/// Errors that could occur when deleting an upload
#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("an upload at the specified id was not found")]
    NotFound,
    #[error("incorrect delete key")]
    IncorrectKey,

    #[error("internal server error")]
    InternalServer(#[from] anyhow::Error),
}
impl From<DeleteUploadError> for DeleteError {
    fn from(err: DeleteUploadError) -> Self {
        match err {
            DeleteUploadError::NotFound => Self::NotFound,
            e => Self::InternalServer(anyhow::Error::new(e).context("error deleting upload")),
        }
    }
}
impl IntoResponse for DeleteError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::IncorrectKey => StatusCode::FORBIDDEN,
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        self.into_response_with_code(code)
    }
}

/// Delete an upload
///
/// # Request
///
/// `DELETE /api/v1/delete/:filename`
///
/// ## Body
///
/// Should contain the key used to delete the upload (`delete_key` in
/// [`UploadResponse`]).
///
/// [`UploadResponse`]: super::upload::UploadResponse::delete_key
///
/// # Response
///
/// ## Success
///
/// - 200 OK
/// - JSON body created from [`DeleteResponse`]
#[instrument(skip(state))]
pub async fn delete(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
    key: String,
) -> Result<impl IntoResponse, DeleteError> {
    let key = key.trim();
    event!(Level::DEBUG, "reading upload metadata");
    let metadata = state
        .backend
        .read_upload_metadata(&id)
        .await
        .map_err(|e| match e {
            OpenUploadError::NotFound(_) => DeleteError::NotFound,
            e => DeleteError::InternalServer(
                anyhow::Error::new(e).context("error reading upload metadata"),
            ),
        })?;
    if metadata.is_expired() {
        event!(
            Level::INFO,
            "upload was already expired anyway, deleting and sending NotFound response"
        );
        state.backend.delete_upload(&id).await?;
        return Err(DeleteError::NotFound);
    }
    if metadata.delete_key != key {
        event!(Level::INFO, "provided delete key was incorrect");
        return Err(DeleteError::IncorrectKey);
    }

    event!(Level::DEBUG, "delete key was correct; deleting upload");
    state.backend.delete_upload(&id).await?;

    event!(Level::INFO, id, "successfully deleted upload");
    Ok(Json(DeleteResponse { id }))
}
