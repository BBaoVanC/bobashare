use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use thiserror::Error;

use super::ApiErrorExt;
use crate::AppState;

/// Errors that could occur when deleting an upload
#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("internal server error")]
    InternalServer(#[from] anyhow::Error),
}
impl IntoResponse for DeleteError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        self.into_response_with_code(code)
    }
}

pub async fn delete(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, DeleteError> {
    // let upload = state.backend.
    todo!();
    Ok(())
}
