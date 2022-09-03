use std::path::PathBuf;

use thiserror::Error;

use super::{LatestUploadFormat, UploadMetadata};
use crate::backend::{Upload, UploadFile};

/// Convert the deserialized metadata into the actual type used by the rest of
/// the code.
impl Upload {
    // TODO: proper error handling of zero-length files array
    pub fn new_migrated(name: String, upload: UploadMetadata) -> Result<Self, ParseUploadError> {
        #[allow(unused_mut)]
        let mut current = upload;

        #[allow(clippy::never_loop)]
        loop {
            match current {
                UploadMetadata::V1(data) => break Upload::from_latest(name, data),
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseUploadError {
    #[error("the files array is empty")]
    ZeroFiles,
}

impl Upload {
    pub fn from_latest(
        name: String,
        upload: LatestUploadFormat,
    ) -> Result<Self, ParseUploadError> {
        let files = upload
            .files
            .into_iter()
            .map(|f| UploadFile {
                path: f.path,
                filename: f.filename,
                mimetype: f.mimetype,
                size: f.size,
            })
            .collect::<Vec<UploadFile>>();

        if files.is_empty() {
            return Err(ParseUploadError::ZeroFiles);
        }

        Ok(Self {
            url: name,
            total_size: upload.total_size,
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files,
        })
    }
}
