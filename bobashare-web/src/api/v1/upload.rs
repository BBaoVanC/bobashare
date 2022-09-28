use std::{sync::Arc, io};

use axum::{
    extract::{multipart::MultipartError, Multipart},
    response::{IntoResponse, Response, ErrorResponse},
    Extension, Json,
};
use bobashare::storage::file::CreateUploadError;
use chrono::{Duration, DateTime, Utc};
use hyper::StatusCode;
use serde::Serialize;
use thiserror::Error;

use crate::AppState;
use axum::response::Result;
// use super::Result;

// #[derive(Debug, Error, Serialize)]
// pub enum UploadError {
//     #[error("error while reading multipart form data")]
//     MultipartError(#[from] MultipartError),
//     #[error("error while doing i/o")]
//     IoError(#[from] io::Error)
// }

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum UploadResponse {
    Success {
        url: String,
        /// expiration date in RFC 3339 format
        expiry_date: DateTime<Utc>,
    },
}

#[derive(Debug, Serialize)]
pub struct UploadInternalError {
    message: String,
    #[serde(skip)]
    code: StatusCode
}
impl From<io::Error> for UploadInternalError {
    fn from(err: io::Error) -> Self {
        Self {
            message: err.to_string(),
            code: match err.kind() {
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }
}
impl From<MultipartError> for UploadInternalError {
    fn from(err: MultipartError) -> Self {
        Self {
            message: err.to_string(),
            code: StatusCode::BAD_REQUEST,
        }
    }
}
impl From<CreateUploadError> for UploadInternalError {
    fn from(err: CreateUploadError) -> Self {
        Self {
            message: err.to_string(),
            code: match err {
                CreateUploadError::AlreadyExists => StatusCode::FORBIDDEN,
                CreateUploadError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }
}
// impl<T: ToString> From<T> for UploadError {
//     fn from(err: T) -> Self {
//         Self { message: err.to_string(), code: StatusCode::INTERNAL_SERVER_ERROR }
//     }
// }

/// Accepts: `multipart/form-data`
///
/// Each form field should be a file to upload. The `name` header is ignored.
pub async fn post(state: Extension<Arc<AppState>>, mut form: Multipart) -> Result<Json<UploadResponse>, UploadInternalError> {
    // need function to set duration after the fact
    let mut name: Option<String> = None;
    let mut duration: Option<Duration> = None;
    let mut files = Vec::new();

    while let Some(field) = form.next_field().await? {
        field.
    }


    let mut upload = state
        .backend
        .create_upload("abc123xyz", Some(Duration::hours(1)))
        .await?;
        // .await
        // .map_err(|e| match e {
        //     CreateUploadError::AlreadyExists => {
        //         (StatusCode::FORBIDDEN, String::from("already exists"))
        //     }
        //     CreateUploadError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        // });
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
