//! Version 1

use chrono::prelude::*;
use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::IntoMetadataError;
use crate::storage::upload::UploadFile;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UploadV1 {
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFileV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UploadFileV1 {
    pub path: RelativePathBuf,
    pub filename: String,
    pub mimetype: String,
}

impl UploadFileV1 {
    pub fn from_file(file: UploadFile) -> Self {
        Self {
            path: file.path,
            filename: file.filename,
            mimetype: file.mimetype,
        }
    }
}
