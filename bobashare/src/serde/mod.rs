use std::path::PathBuf;

use relative_path::FromPathError;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

use self::v1::{UploadFileV1, UploadV1};
use super::storage::upload::Upload;
use crate::storage::upload::UploadFile;

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

#[derive(Debug, Error)]
pub enum IntoMetadataError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
    #[error("error while converting path {0} to relative path")]
    ToRelativeError(#[from] FromPathError),
}
impl UploadMetadata {
    pub fn from_upload(upload: Upload) -> Self {
        let mut files = Vec::with_capacity(upload.files.len());
        for file in upload.files.into_iter() {
            files.push(UploadFileV1::from_file(file));
        }

        Self::V1(UploadV1 {
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files,
        })
    }
}

impl UploadMetadata {
    pub fn into_migrated_upload(path: PathBuf, metadata: UploadMetadata) -> Upload {
        match metadata {
            // latest
            Self::V1(data) => Upload {
                path,
                creation_date: data.creation_date,
                expiry_date: data.expiry_date,
                files: data
                    .files
                    .into_iter()
                    .map(|f| UploadFile {
                        filename: f.filename,
                        mimetype: f.mimetype,
                        path: f.path,
                    })
                    .collect::<Vec<_>>(),
            },
        }
    }
}
