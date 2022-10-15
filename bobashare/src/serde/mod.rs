//! Methods to serialize [`Upload`]s using [`serde_json`]

use mime::Mime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use self::v1::UploadV1;
use super::storage::upload::Upload;

#[cfg(test)]
mod tests;

pub mod v1;

/// The latest upload metadata version
pub type LatestUploadMetadata = UploadV1;

/// All the versions of upload metadata
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "version")]
#[non_exhaustive]
pub enum UploadMetadata {
    #[serde(rename = "1")]
    V1(UploadV1),
}
impl UploadMetadata {
    /// Convert an upload into the latest metadata version
    pub fn from_upload(upload: Upload) -> Self {
        Self::V1(LatestUploadMetadata {
            filename: upload.filename,
            mimetype: upload.mimetype.to_string(),
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            delete_key: upload.delete_key,
        })
    }
}

#[derive(Debug, Error)]
pub enum MigrateErrorV1 {
    #[error("error parsing `mimetype` field")]
    ParseMime(#[from] mime::FromStrError),
}

/// Errors that could occur while migrating an upload (during deserialization)
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MigrateError {
    #[error("error migrating from V1")]
    V1(#[from] MigrateErrorV1),
}
impl UploadMetadata {
    // TODO: maybe migrating should be a separate task and it should immediately
    // error if not already migrated
    pub fn into_migrated_upload(
        id: String,
        metadata: UploadMetadata,
    ) -> Result<Upload, MigrateError> {
        Ok(match metadata {
            // latest
            Self::V1(data) => Upload {
                id,
                filename: data.filename,
                mimetype: data
                    .mimetype
                    .parse::<Mime>()
                    .map_err(MigrateErrorV1::from)?,
                creation_date: data.creation_date,
                expiry_date: data.expiry_date,
                delete_key: data.delete_key,
            },
        })
    }
}
