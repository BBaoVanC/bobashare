use std::{fmt::Display, path::PathBuf};

use relative_path::FromPathError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;
use tokio::fs::File;

use self::v1::{UploadV1, UploadFileV1};

use super::storage::upload::{Upload, UploadFile};

#[cfg(test)]
mod tests;

pub mod v1;

pub type LatestUploadMetadata = UploadV1;

// TODO: maybe add an index to FileBackend with expiry dates
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum UploadMetadata {
    #[serde(rename = "1")]
    V1(UploadV1),
}

#[derive(Debug)]
pub struct UploadMetadataHandle {
    pub metadata: UploadMetadata,
    file: File,
}
impl UploadMetadataHandle {
    pub async fn new(metadata: UploadMetadata, path: PathBuf) -> Result<Self, io::Error> {
        let file = File::create(path.join("metadata.json")).await?;

        Ok(Self {
            metadata,
            file,
        })
    }
}

#[derive(Debug, Error)]
pub enum FromMetadataError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
    #[error("error while converting path {0} to relative path")]
    ToRelativeError(#[from] FromPathError)
}
impl UploadMetadata {
    pub async fn new(upload: Upload) -> Result<Self, FromMetadataError> {
        Ok(Self::V1(LatestUploadMetadata::from_latest(upload).await?))
    }
}
impl LatestUploadMetadata {
    pub async fn from_latest(upload: Upload) -> Result<Self, FromMetadataError> {
        let mut files = Vec::with_capacity(upload.files.len());
        for file in upload.files.clone() { // TODO: get rid of this clone
            files.push(UploadFileV1::from_file(file).await?);
        }

        Ok(Self {
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files,
        })
    }
}
