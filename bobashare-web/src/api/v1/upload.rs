use std::{io, sync::Arc};

use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::Result,
    Extension, Json,
};
use bobashare::storage::file::CreateUploadError;
use chrono::{DateTime, Duration, Utc};
use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;

use crate::AppState;

use super::ApiErrorV1;

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub struct UploadResponse {
    url: String,
    /// expiration date in RFC 3339 format
    expiry_date: DateTime<Utc>,
}

/// Accepts: `multipart/form-data`
///
/// Each form field should be a file to upload. The `name` header is ignored.
pub async fn post(
    state: Extension<Arc<AppState>>,
    mut form: Multipart,
) -> Result<Json<UploadResponse>, ApiErrorV1> {
    // need function to set duration after the fact
    let mut name: Option<String> = None;
    let mut duration: Option<Duration> = None;
    // let mut files = Vec::new();

    // while let Some(field) = form.next_field().await? {
    //     match field.name().ok_or(UploadInternalError {
    //         code: StatusCode::BAD_REQUEST,
    //         message: String::from("received a multipart field with no name"),
    //     })? {

    //     }
    // }

    let mut upload = state
        .backend
        .create_upload("abc123xyz", Some(Duration::hours(1)))
        .await?;
    // .await
    // .map_err(|e| match e {
    //     CreateUploadError::AlreadyExists => {
    //         (StatusCode::FORBIDDEN, String::from("already exists"))
    //     }
    //     CreateUploadError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR,
    // e.to_string()), });
    let mut i = 0;
    // while let Some(mut field) = form.next_field().await? {
    while let Some(field) = form.next_field().await? {
        i += 1; // starts at 1
        if field.content_type().is_none() {
            continue;
        }
        if field.file_name().is_none() {
            continue;
        }
        let _mimetype = field.content_type().unwrap();
        let _filename = field.file_name().unwrap();

        // let name_on_disk = format!("{:0<4}", 5);
        println!("{}", i); // TODO: remove this
        todo!();
    }

    todo!();
    // (StatusCode::CREATED, "Created")
}
