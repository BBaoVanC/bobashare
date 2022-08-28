//! Module to serialize upload metadata so it can be stored on the server.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Upload;

#[cfg(test)]
mod tests;

/// Version 1
// pub mod v1;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum UploadMetadataVersion {
    #[serde(rename = "1")]
    V1,
}
impl UploadMetadataVersion {
    const LATEST: Self = Self::V1;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UploadMetadata {
    version: UploadMetadataVersion,
    upload: Value,
}
impl UploadMetadata {
    pub fn from_upload(upload: &Upload) -> Result<Self, serde_json::Error> {
        Ok(Self {
            version: UploadMetadataVersion::LATEST,
            upload: upload.serialize(serde_json::value::Serializer)?
        })
    }
}
// impl<'de> Deserialize<'de> for UploadMetadata {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: serde::Deserializer<'de> {
//                 deserializer.
//     }
// }
