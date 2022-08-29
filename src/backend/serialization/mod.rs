//! All the code to (de)serialize upload metadata.

use serde::{Deserialize, Serialize};

use self::v1::{UploadV1, UploadFileV1};

use super::{Upload, UploadContents, UploadFile};

pub mod migrate;
pub mod v1;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum UploadMetadata {
    #[serde(rename = "1")]
    V1(UploadV1),
}
impl UploadMetadata {
    pub fn new_latest(upload: Upload) -> Self {
        Self::V1(upload.into())
    }
}
