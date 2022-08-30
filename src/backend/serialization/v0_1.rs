use std::path::PathBuf;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::v0::{UploadFileV0, UploadV0};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadV0_1 {
    pub size_total: u64,
    pub date_created: DateTime<Utc>,
    pub date_expires: DateTime<Utc>,
    pub files: Vec<UploadFileV0_1>,
    pub coolness: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadFileV0_1 {
    pub path: PathBuf,
    pub filename: String,
    pub size_bytes: u64,
}

impl From<UploadV0> for UploadV0_1 {
    fn from(upload: UploadV0) -> Self {
        Self {
            size_total: upload.size_total,
            date_created: upload.date_created,
            date_expires: upload.date_expires,
            coolness: upload.coolness,
            files: upload
                .files
                .into_iter()
                .map(Into::into)
                .collect::<Vec<UploadFileV0_1>>(),
        }
    }
}
impl From<UploadFileV0> for UploadFileV0_1 {
    fn from(file: UploadFileV0) -> Self {
        Self {
            path: PathBuf::from(file.path),
            filename: file.filename,
            size_bytes: file.size_bytes,
        }
    }
}
