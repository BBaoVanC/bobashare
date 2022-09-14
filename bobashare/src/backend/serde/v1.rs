//! Version 1

use chrono::prelude::*;
use relative_path::{RelativePathBuf, FromPathError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{fs, io};

use crate::backend::storage::upload::UploadFile;

use super::{IntoMetadataError};

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
    pub size: u64,
}

impl UploadFileV1 {
    pub async fn from_file(file: UploadFile) -> Result<Self, IntoMetadataError> {
        let size = fs::metadata(&file.path).await?.len();

        Ok(Self {
            path: RelativePathBuf::from_path(file.path)?,
            filename: file.filename,
            mimetype: file.mimetype,
            size,
        })
    }
}
