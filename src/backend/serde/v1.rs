//! Version 1

use chrono::prelude::*;
use relative_path::{RelativePathBuf, FromPathError};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{fs, io};

use crate::backend::storage::upload::UploadFile;

use super::MigrateMetadataError;

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

#[derive(Debug, Error)]
pub enum MigrateV1Error {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
    #[error("error converting the contained PathBuf to a RelativePathBuf")]
    RelativePathError(#[from] FromPathError)
}
impl From<MigrateV1Error> for MigrateMetadataError {
    fn from(error: MigrateV1Error) -> Self {
        Self::V1(error)
    }
}

impl UploadFileV1 {
    pub async fn from_file(file: UploadFile) -> Result<Self, MigrateV1Error> {
        let size = fs::metadata(&file.path).await?.len();

        Ok(Self {
            path: RelativePathBuf::from_path(file.path)?,
            filename: file.filename,
            mimetype: file.mimetype,
            size,
        })
    }
}
