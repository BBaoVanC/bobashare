use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use thiserror::Error;
use tokio::io;

use super::ApiErrorExt;
use crate::AppState;

/// Errors that could occur when deleting an upload
#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("an upload at the specified id was not found")]
    NotFound,

    #[error("internal server error")]
    InternalServer(#[from] anyhow::Error),
}
impl IntoResponse for DeleteError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        self.into_response_with_code(code)
    }
}

pub async fn delete(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
    #[allow(unused_variables)] body: String,
) -> Result<impl IntoResponse, DeleteError> {
    if false {
        state
            .backend
            .delete_upload(id)
            .await
            .map_err(|e| match e.kind() {
                io::ErrorKind::NotFound => DeleteError::NotFound,
                _ => DeleteError::InternalServer(
                    anyhow::Error::new(e).context("error deleting upload"),
                ),
            })?;
    }
    Ok(())
}
