//! Methods to serialize [`Upload`]s using [`serde_json`]

use displaydoc::Display;
use mime::Mime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use self::v0::UploadV0;
use super::storage::upload::Upload;

#[cfg(test)]
mod tests;

/// The latest upload metadata version
pub type LatestUploadMetadata = UploadV0;

/// All the versions of upload metadata
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "version")]
#[non_exhaustive]
pub enum UploadMetadata {
    #[serde(rename = "0")]
    V0(UploadV0),

    #[serde(other)]
    Unknown,
}
impl UploadMetadata {
    /// Convert an upload into the latest metadata version
    pub fn from_upload(upload: Upload) -> Self {
        Self::V0(LatestUploadMetadata {
            filename: upload.filename,
            mimetype: upload.mimetype.to_string(),
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            delete_key: upload.delete_key,
        })
    }
}

/// Errors when migrating from [`UploadV0`]
#[derive(Debug, Error, Display)]
pub enum MigrateErrorV0 {
    /// error parsing `mimetype` field
    ParseMime(#[from] mime::FromStrError),
}

/// Errors that could occur while migrating an upload (during deserialization)
#[derive(Debug, Error, Display)]
#[non_exhaustive]
pub enum MigrateError {
    /// error migrating from V0
    // TODO: should we say this from perspective of migrating FROM 0 to X
    // or migrating TO X from 0
    V0(#[from] MigrateErrorV0),

    /// unknown upload version
    UnknownVersion,
}
impl UploadMetadata {
    // TODO: maybe migrating should be a separate task and it should immediately
    // error if not already migrated
    /// Convert [`UploadMetadata`] into an [`Upload`], migrating it if needed
    ///
    /// Returns a tuple on success containing the migrated upload and whether it
    /// was migrated (or if it was already the latest version)
    pub fn into_migrated_upload(self, id: String) -> Result<(Upload, bool), MigrateError> {
        Ok(match self {
            Self::Unknown => return Err(MigrateError::UnknownVersion),

            // latest
            Self::V0(data) => (
                Upload {
                    id,
                    filename: data.filename,
                    mimetype: data
                        .mimetype
                        .parse::<Mime>()
                        .map_err(MigrateErrorV0::from)?,
                    creation_date: data.creation_date,
                    expiry_date: data.expiry_date,
                    delete_key: data.delete_key,
                },
                false, // already latest
            ),
        })
    }
}

pub mod v0 {
    //! Version 1
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    /// A serializable form of [`Upload`], version 1
    ///
    /// [`Upload`]: crate::storage::upload::Upload
    #[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
    pub struct UploadV0 {
        /// name of the uploaded file
        pub filename: String,
        /// MIME type of the uploaded file
        pub mimetype: String,
        /// date the upload was created
        pub creation_date: DateTime<Utc>,
        /// date the upload expires, or [`None`] if never
        pub expiry_date: Option<DateTime<Utc>>,
        /// secret key needed to delete the upload before its expiry
        pub delete_key: String,
    }
}
