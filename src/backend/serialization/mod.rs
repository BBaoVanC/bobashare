//! All the code to (de)serialize upload metadata.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Upload;

pub mod v1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum UploadMetadataVersion {
    #[serde(rename = "1")]
    V1,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadMetadata {
    version: UploadMetadataVersion,
    #[serde(flatten)]
    data: Value,
}

impl From<UploadMetadata> for Upload {
    fn from(metadata: UploadMetadata) -> Self {
        match metadata.version {
            UploadMetadataVersion::V1 => Self {

            }
        }
    }
}
