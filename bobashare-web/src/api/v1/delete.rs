//! API to delete an upload

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use bobashare::storage::file::OpenUploadError;
use serde::Serialize;
use tracing::{event, instrument, Level};
use utoipa::ToSchema;

use super::ApiError;
use crate::AppState;

/// the upload was deleted successfully
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DeleteResponse {
    /// the ID of the deleted upload
    #[schema(example = "Hk6Shy0Q")]
    pub id: String,
}

/// Delete an upload
#[instrument(skip(state))]
#[utoipa::path(
    delete,
    context_path = "/api/v1",
    path = "/delete/{id}",
    params(
        ("id" = String, Path, example = "Hk6Shy0Q"),
    ),
    request_body(content = inline(String), description = "`delete_key` of the upload"),
    responses(
        (status = 200, body = DeleteResponse, description = "the upload was deleted successfully",
        ),
        (status = 404, body = ApiError, description = "the upload was not found",
            example = json!({
                "message": "an upload at the specified id was not found",
            }),
        ),
        (status = 403, body = ApiError, description = "incorrect delete key provided",
            example = json!({
                "message": "incorrect delete key",
            }),
        ),
        (status = 500, body = ApiError, description = "internal server error",
            example = json!({
                "message": "internal server error: error deleting upload file",
            }),
        ),
    )
)]
// TODO: should this return 204 No Content and empty body?
pub async fn delete(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
    key: String,
) -> Result<impl IntoResponse, ApiError> {
    let key = key.trim();
    event!(Level::DEBUG, "reading upload metadata");
    let metadata = state
        .backend
        .read_upload_metadata(&id)
        .await
        .map_err(|e| match e {
            OpenUploadError::NotFound(_) => ApiError::NotFound,
            e => ApiError::InternalServer { source: e.into() },
        })?;
    if metadata.is_expired() {
        event!(
            Level::INFO,
            "upload was already expired anyway, deleting and sending NotFound response"
        );
        state
            .backend
            .delete_upload(&id)
            .await
            .map_err(|e| ApiError::InternalServer { source: e.into() })?;
        return Err(ApiError::NotFound);
    }
    if metadata.delete_key != key {
        event!(Level::INFO, "provided delete key was incorrect");
        return Err(ApiError::IncorrectKey);
    }

    event!(Level::DEBUG, "delete key was correct; deleting upload");
    state
        .backend
        .delete_upload(&id)
        .await
        .map_err(|e| ApiError::InternalServer { source: e.into() })?;

    event!(Level::INFO, id, "successfully deleted upload");
    Ok(Json(DeleteResponse { id }))
}
