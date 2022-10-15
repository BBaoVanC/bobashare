//! API to get metadata about an upload

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};
use bobashare::storage::file::OpenUploadError;
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;
use tracing::{event, instrument, Level};

use super::ApiErrorExt;
use crate::AppState;

/// Successful upload info API response
#[derive(Debug, Clone, Serialize)]
pub struct InfoResponse {
    /// ID of the upload
    pub id: String,
    /// URL of the upload
    pub url: String,
    /// direct URL to download the upload file
    pub direct_url: String,
    /// filename of the uploaded file
    pub filename: String,
    /// MIME type of the file
    pub mimetype: String,
    /// date the upload was created
    pub creation_date: DateTime<Utc>,
    /// date the upload expires, or None if it never expires
    pub expiry_date: Option<DateTime<Utc>>,
    // don't accidentally send `delete_key` lol
}

/// Errors when querying info about an upload
#[derive(Debug, Error)]
pub enum InfoError {
    #[error("an upload at the specified id was not found")]
    NotFound,

    #[error("internal server error")]
    InternalServer(#[from] anyhow::Error),
}
impl IntoResponse for InfoError {
    fn into_response(self) -> Response {
        let code = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        self.into_response_with_code(code)
    }
}

/// Get information (metadata) about an upload
///
/// # Request
///
/// - `GET /api/v1/info/:id`
///
/// # Response
///
/// ## Success
///
/// - 200 OK
/// - JSON body created from [`InfoResponse`]
#[instrument(skip(state))]
pub async fn info(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, InfoError> {
    event!(Level::DEBUG, id, "reading upload metadata");
    let metadata = state
        .backend
        .read_upload_metadata(&id)
        .await
        .map_err(|e| match e {
            OpenUploadError::NotFound(_) => InfoError::NotFound,
            e => InfoError::InternalServer(
                anyhow::Error::new(e).context("error reading upload metadata"),
            ),
        })?;

    let url = state.base_url.join(&id).unwrap().to_string();
    let direct_url = state.raw_url.join(&id).unwrap().to_string();
    event!(Level::INFO, "successfully queried upload metadata");
    Ok(Json(InfoResponse {
        id,
        url,
        direct_url,
        filename: metadata.filename,
        mimetype: metadata.mimetype.to_string(),
        creation_date: metadata.creation_date,
        expiry_date: metadata.expiry_date,
    }))
}
