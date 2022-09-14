use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use self::v1::{UploadV1, MigrateV1Error, UploadFileV1};

use super::storage::upload::{Upload, UploadFile};

#[cfg(test)]
mod tests;

pub mod v1;

pub type LatestUploadMetadata = UploadV1;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum UploadMetadata {
    #[serde(rename = "1")]
    V1(UploadV1),
}
impl UploadMetadata {
    pub async fn new(upload: Upload) -> Result<Self, MigrateMetadataError> {
        Ok(Self::V1(LatestUploadMetadata::from_latest(upload).await?))
    }
}

#[derive(Debug, Error)]
pub enum MigrateMetadataError {
    V1(MigrateV1Error),
}
impl Display for MigrateMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MigrateMetadataError::V1(e) => format!("V1::MigrateV1Error{{{e}}}"),
        })
    }
}

impl LatestUploadMetadata {
    pub async fn from_latest(upload: Upload) -> Result<Self, MigrateMetadataError> {
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
