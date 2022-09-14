use relative_path::FromPathError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{fs, io, io::AsyncWriteExt};

use self::v1::{UploadFileV1, UploadV1};
use super::storage::upload::Upload;

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

#[derive(Debug, Error)]
pub enum IntoMetadataError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
    #[error("error while converting path {0} to relative path")]
    ToRelativeError(#[from] FromPathError),
}
impl UploadMetadata {
    pub async fn from_upload(upload: Upload) -> Result<Self, IntoMetadataError> {
        let mut files = Vec::with_capacity(upload.files.len());
        for file in upload.files.clone() {
            // TODO: get rid of this clone
            files.push(UploadFileV1::from_file(file).await?);
        }

        Ok(Self::V1(UploadV1 {
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files,
        }))
    }
}

#[derive(Debug, Error)]
pub enum SerializeMetadataError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
    #[error("error converting Upload to UploadMetadata")]
    FromMetadataError(#[from] IntoMetadataError),
    #[error("error from serde_json")]
    SerdeError(#[from] serde_json::Error),
}
impl Upload {
    pub async fn save(self) -> Result<(), SerializeMetadataError> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.path.join("metadata.json"))
            .await?;
        file.write_all(
            serde_json::to_string(&UploadMetadata::from_upload(self).await?)?.as_bytes(),
        )
        .await?;
        Ok(())
    }
}
