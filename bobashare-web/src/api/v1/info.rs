//! API to get metadata about an upload

use std::{error::Error as StdError, sync::Arc};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use bobashare::storage::file::OpenUploadError;
use chrono::{DateTime, Utc};
use displaydoc::Display;
use hyper::StatusCode;
use serde::Serialize;
use tracing::{event, instrument, Level};
use utoipa::ToSchema;

use super::ApiError;
use crate::AppState;

/// Successful upload info API response
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct InfoResponse {
    /// ID of the upload
    #[schema(example = "Hk6Shy0Q")]
    pub id: String,
    /// URL of the upload
    #[schema(example = "http://localhost:3000/Hk6Shy0Q")]
    pub url: String,
    /// direct URL to download the upload file
    #[schema(example = "http://localhost:3000/raw/Hk6Shy0Q")]
    pub direct_url: String,
    /// filename of the uploaded file
    #[schema(example = "example.txt")]
    pub filename: String,
    /// MIME type of the file
    #[schema(example = "text/plain")]
    pub mimetype: String,
    /// date the upload was created
    #[schema(example = "2022-11-12T23:20:09.008416131Z", value_type = String, format = DateTime)]
    pub creation_date: DateTime<Utc>,
    /// date the upload expires, or None if it never expires
    #[schema(example = "2022-11-13T23:20:09.008416131Z", value_type = String, format = DateTime, nullable)]
    pub expiry_date: Option<DateTime<Utc>>,
    // don't accidentally send `delete_key` lol
}

/// Errors when querying info about an upload
#[derive(Debug, Display, Serialize)]
pub enum InfoError {
    /// an upload at the specified id was not found
    NotFound,

    /// internal server error: {source}
    InternalServer {
        #[serde(skip)]
        source: Box<dyn StdError>,
    },
}
impl StdError for InfoError {}
impl From<InfoError> for ApiError {
    fn from(err: InfoError) -> Self {
        let code = match err {
            InfoError::NotFound => StatusCode::NOT_FOUND,
            InfoError::InternalServer { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            code,
            message: err.to_string(),
            source: Some(err.into()),
        }
    }
}

#[instrument(skip(state))]
#[utoipa::path(
    context_path = "/api/v1",
    get,
    path = "/info/{id}",
    responses(
        (status = 200, body = InfoResponse, description = "success",
            example = json!({
                "id": "Hk6Shy0Q",
                "url": "http://localhost:3000/Hk6Shy0Q",
                "direct_url": "http://localhost:3000/raw/Hk6Shy0Q",
                "filename": "example.txt",
                "mimetype": "text/plain",
                "creation_date": "2022-11-12T23:20:09.008416131Z",
                "expiry_date": "2022-11-13T23:20:09.008416131Z"
            })
        ),
        (status = 404, body = ApiError, description = "upload not found",
            example = json!({
                "message": "an upload at the specified id was not found"
            })
        ),
        (status = 500, body = ApiError, description = "internal server error",
            example = json!({
                "message": "internal server error: error deserializing upload metadata"
            })
        )
    ),
)]
pub async fn info(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    event!(Level::DEBUG, id, "reading upload metadata");
    let metadata = state
        .backend
        .read_upload_metadata(&id)
        .await
        .map_err(|e| match e {
            OpenUploadError::NotFound(_) => InfoError::NotFound,
            e => InfoError::InternalServer { source: e.into() },
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
