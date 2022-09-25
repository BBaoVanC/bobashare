use std::{path::PathBuf, ffi::OsString};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

use crate::storage::upload::UploadFile;

use self::v1::{UploadFileV1, UploadV1};
use super::storage::upload::Upload;

// #[cfg(test)]
// mod tests;

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
}
impl UploadMetadata {
    pub fn from_upload(upload: Upload) -> Self {
        let mut files = Vec::with_capacity(upload.files.len());
        for (path, file) in upload.files.into_iter() {
            // files.push(UploadFileV1::from_file(file));
            files.push(UploadFileV1 {
                path: path.to_string_lossy().to_string(),
                filename: file.filename,
                mimetype: file.mimetype,
            })
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
                creation_date: data.creation_date,
                expiry_date: data.expiry_date,
                files: data
                    .files
                    .into_iter()
                    .map(|f| (OsString::from(f.path.clone()), UploadFile { path: path.join(OsString::from(f.path)), filename: f.filename, mimetype: f.mimetype })) // From<UploadFileV1> for UploadFile
                    .collect(),
                path,
            },
        }
    }
}
