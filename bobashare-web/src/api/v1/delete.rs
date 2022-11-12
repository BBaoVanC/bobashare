//! API to delete an upload

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use bobashare::storage::file::{DeleteUploadError, OpenUploadError};
use displaydoc::Display;
use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;
use tracing::{event, instrument, Level};
use utoipa::ToSchema;

use super::ApiErrorExt;
use crate::AppState;

/// API response after deleting an upload successfully
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DeleteResponse {
    /// The ID of the deleted upload
    #[schema(example = "Hk6Shy0Q")]
    pub id: String,
}

/// Errors that could occur when deleting an upload
#[derive(Debug, Error, Display, ToSchema)]
pub enum DeleteError {
    /// an upload at the specified id was not found
    #[schema(title = "NotFound")]
    NotFound,
    /// incorrect delete key
    IncorrectKey,

    /// internal server error
    InternalServer(#[from] #[schema(value_type = Object)] anyhow::Error),
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
// impl ToResponse for DeleteError {
//     fn response() -> (String, utoipa::openapi::Response) {

//     }
// }

/// Delete an upload
#[instrument(skip(state))]
#[utoipa::path(
    delete,
    context_path = "/api/v1",
    path = "/delete/{id}",
    params(
        ("id" = String, Path, description = "ID of the upload to delete", example = "Hk6Shy0Q"),
    ),
    request_body(content = inline(String), description = "`delete_key` of the upload"),
    responses(
        (status = 200, body = DeleteResponse, description = "deleted successfully"),
        (status = 404, body = DeleteError, description = "upload not found"),
        (status = 403, body = DeleteError, description = "incorrect delete key"),
        (status = 500, body = DeleteError, description = "internal server error"),
    )
)]
// TODO: should this return 204 No Content and empty body?
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
