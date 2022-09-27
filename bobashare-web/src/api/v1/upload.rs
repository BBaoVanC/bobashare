use std::sync::Arc;

use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::{IntoResponse, Response},
    Extension, Json,
};
use bobashare::storage::file::CreateUploadError;
use chrono::Duration;
use hyper::StatusCode;
use thiserror::Error;

use super::Result;
use crate::AppState;

/// Accepts: `multipart/form-data`
///
/// Each form field should be a file to upload. The `name` header is ignored.
pub async fn post(
    state: Extension<Arc<AppState>>,
    mut form: Multipart,
) -> Result {
    let mut _upload = state
        .backend
        .create_upload("abc123xyz", Some(Duration::hours(1)))
        .await
        .map_err(|e| match e {
            CreateUploadError::AlreadyExists => {
                (StatusCode::FORBIDDEN, String::from("already exists"))
            }
            CreateUploadError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        });
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

    (StatusCode::CREATED, "Created")
}
