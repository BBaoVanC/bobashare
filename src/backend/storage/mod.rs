//! Module to serialize upload metadata so it can be stored on the server.

use serde::{Deserialize, Serialize};

use self::v1::UploadV1;

#[cfg(test)]
mod tests;

/// Version 1
pub mod v1;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
pub enum UploadMetadata {
    #[serde(rename = "1")]
    V1(UploadV1),
}
