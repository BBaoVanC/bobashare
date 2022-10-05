use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

use self::v1::UploadV1;
use super::storage::upload::Upload;

// #[cfg(test)]
// mod tests;

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
    #[error("error while doing i/o: {0}")]
    Io(#[from] io::Error),
}
impl UploadMetadata {
    pub fn from_upload(upload: Upload) -> Self {
        Self::V1(UploadV1 {
            size: upload.size,
            filename: upload.filename,
            mimetype: upload.mimetype,
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
        })
    }
}

impl UploadMetadata {
    pub fn into_migrated_upload(url: String, metadata: UploadMetadata) -> Upload {
        match metadata {
            // latest
            Self::V1(data) => Upload {
                url,
                size: data.size,
                filename: data.filename,
                mimetype: data.mimetype,
                creation_date: data.creation_date,
                expiry_date: data.expiry_date,
            },
        }
    }
}
