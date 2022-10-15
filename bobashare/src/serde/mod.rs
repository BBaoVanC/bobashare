use mime::Mime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use self::v1::UploadV1;
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
impl UploadMetadata {
    pub fn from_upload(upload: Upload) -> Self {
        Self::V1(UploadV1 {
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
    #[error("error parsing `mimetype` field: {0}")]
    ParseMime(#[from] mime::FromStrError),
}
#[derive(Debug, Error)]
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
