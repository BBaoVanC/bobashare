//! API to delete an upload

use std::{error::Error, sync::Arc};

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use bobashare::storage::file::{DeleteUploadError, OpenUploadError};
use displaydoc::Display;
use hyper::StatusCode;
use serde::Serialize;
use tracing::{event, instrument, Level};
use utoipa::ToSchema;

use crate::AppState;

/// the upload was deleted successfully
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DeleteResponse {
    /// the ID of the deleted upload
    #[schema(example = "Hk6Shy0Q")]
    pub id: String,
}

/// errors that could occur when deleting an upload
#[derive(Debug, Display, Serialize, ToSchema)]
#[serde(tag = "error")]
pub enum DeleteError {
    /// an upload at the specified id was not found
    NotFound,
    /// incorrect delete key
    IncorrectKey,

    /// internal server error: {reason}
    InternalServer {
        reason: String,
        #[serde(skip)]
        extra_context: Box<dyn Error>,
    },
}
impl From<DeleteUploadError> for DeleteError {
    fn from(err: DeleteUploadError) -> Self {
        match err {
            DeleteUploadError::NotFound => Self::NotFound,
            e => Self::InternalServer {
                reason: e.to_string(),
                extra_context: e.into(),
            },
        }
    }
}
impl From<Box<dyn Error>> for DeleteError {
    fn from(err: Box<dyn Error>) -> Self {
        Self::InternalServer {
            reason: err.to_string(),
            extra_context: err,
        }
    }
}
impl IntoResponse for DeleteError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::IncorrectKey => StatusCode::FORBIDDEN,
            Self::InternalServer {
                reason: _,
                extra_context: _,
            } => StatusCode::INTERNAL_SERVER_ERROR,
        };

        if code.is_server_error() {
            event!(Level::ERROR, status = code.as_u16(), error = ?self);
        } else if code.is_client_error() {
            event!(Level::WARN, status = code.as_u16(), error = ?self);
        } else {
            event!(Level::INFO, status = code.as_u16(), error = ?self);
        }

        (code, Json(self)).into_response()
    }
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
        (status = 404, body = DeleteError, description = "the upload was not found",
            example = json!({
                "error": "NotFound",
            }),
        ),
        (status = 403, body = DeleteError, description = "incorrect delete key provided",
            example = json!({
                "error": "IncorrectKey",
            }),
        ),
        (status = 500, body = DeleteError, description = "internal server error",
            example = json!({
                "error": "InternalServer",
                "reason": "some error message",
            }),
        ),
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
            e => (Box::new(e) as Box<dyn Error>).into(),
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
