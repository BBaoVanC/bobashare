use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io;

use self::v1::{UploadFileV1, UploadV1};
use super::storage::upload::Upload;
use crate::storage::upload::UploadFile;

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
    #[error("error while doing i/o: {0}")]
    Io(#[from] io::Error),
}
impl UploadMetadata {
    pub fn from_upload(upload: Upload) -> Self {
        let files = upload
            .files
            .into_iter()
            .map(|(s, f)| {
                (
                    s,
                    UploadFileV1 {
                        filename: f.filename,
                        mimetype: f.mimetype,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

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
                    .map(|(s, f)| {
                        (
                            s,
                            UploadFile {
                                filename: f.filename,
                                mimetype: f.mimetype,
                            },
                        )
                    })
                    .collect(),
                path,
            },
        }
    }
}
