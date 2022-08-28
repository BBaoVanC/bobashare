//! Module to serialize upload metadata so it can be stored on the server.

use serde::{Deserialize, Serialize};

use self::v1::UploadV1;

use super::Upload;

#[cfg(test)]
mod tests;

/// Version 1
pub mod v1;

// https://www.reddit.com/r/rust/comments/5rwe3w/comment/ddaq5lf/
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum UploadMetadata {
    #[serde(rename = "1")]
    V1(UploadV1),
}
impl From<Upload> for UploadMetadata {
    fn from(upload: Upload) -> Self {
        Self::V1(upload.into())
    }
}
