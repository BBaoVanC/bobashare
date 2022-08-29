//! All the code to (de)serialize upload metadata.

use serde::{Deserialize, Serialize};

use self::{v1::{UploadV1, UploadFileV1}, v0::UploadV0, v0_1::UploadV0_1};
use super::{Upload, UploadFile, UploadContents};

pub mod migrate;

pub mod v0;
pub mod v0_1;
pub mod v1;

#[cfg(test)]
mod tests;

pub trait Migrate {
    type Output;

    fn migrate(self) -> Self::Output;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "version")]
/// Main struct that serializes into the metadata stored on disk about an upload.
pub enum UploadMetadata {
    #[serde(rename = "0")]
    V0(UploadV0),
    #[serde(rename = "0.1")]
    V0_1(UploadV0_1),
    #[serde(rename = "1")]
    V1(UploadV1),
}
impl UploadMetadata {
    /// Convert an [`Upload`] into the latest version metadata format so it can be serialized.
    pub fn new(upload: Upload) -> Self {
        Self::V1(upload.into())
    }
}

// Serialization into the latest version
impl From<Upload> for UploadV1 {
    fn from(upload: Upload) -> Self {
        Self {
            total_size: upload.total_size,
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files: match upload.files {
                UploadContents::Single(file) => vec![file.into()],
                UploadContents::Multiple(files) => files
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UploadFileV1>>(),
            },
        }
    }
}
impl From<UploadFile> for UploadFileV1 {
    fn from(file: UploadFile) -> Self {
        Self {
            path: file.path,
            filename: file.filename,
            size: file.size,
        }
    }
}
