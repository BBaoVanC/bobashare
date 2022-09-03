//! All the code to (de)serialize upload metadata.

use serde::{Deserialize, Serialize};

use self::v1::{UploadFileV1, UploadV1};
use super::Upload;

pub mod migrate;

pub mod v1;

#[cfg(test)]
mod tests;

/// The latest format for serialized upload metadata.
pub type LatestUploadFormat = UploadV1;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "version")]
/// Main struct that serializes into the metadata stored on disk about an
/// upload.
// TODO: use https://github.com/Soft/enum-kinds to implement Ord
pub enum UploadMetadata {
    #[serde(rename = "1")]
    V1(UploadV1),
}
impl UploadMetadata {
    /// Convert an [`Upload`] into the latest version metadata format so it can
    /// be serialized.
    pub fn new(upload: Upload) -> Self {
        Self::V1(upload.into())
    }
}

// Serialization into the latest version
impl From<Upload> for LatestUploadFormat {
    fn from(upload: Upload) -> Self {
        Self {
            total_size: upload.total_size,
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files: upload
                .files
                .into_iter()
                .map(Into::into)
                .collect::<Vec<UploadFileV1>>(),
        }
    }
}
