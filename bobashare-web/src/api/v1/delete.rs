//! API to delete an upload

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use bobashare::storage::file::{DeleteUploadError, OpenUploadError};
use displaydoc::Display;
use hyper::StatusCode;
use thiserror::Error;
use tracing::{event, instrument, Level};

use super::ApiErrorExt;
use crate::AppState;

/// Errors that could occur when deleting an upload
#[derive(Debug, Error, Display)]
pub enum DeleteError {
    /// an upload at the specified id was not found
    NotFound,
    /// incorrect delete key
    IncorrectKey,

    /// internal server error
    InternalServer(#[from] anyhow::Error),
}
impl From<DeleteUploadError> for DeleteError {
    fn from(err: DeleteUploadError) -> Self {
        match err {
            DeleteUploadError::NotFound => Self::NotFound,
            e => Self::InternalServer(e.into()),
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
/// `DELETE /api/v1/delete/:id`
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
/// - 204 No Content
#[instrument(skip(state))]
pub async fn delete(
    state: State<&'static AppState>,
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
    Ok(StatusCode::NO_CONTENT)
}
