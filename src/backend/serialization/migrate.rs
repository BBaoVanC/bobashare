use std::path::PathBuf;

use thiserror::Error;

use super::{LatestUploadFormat, UploadMetadata};
use crate::backend::{Upload, UploadContents, UploadFile};

/// Convert the deserialized metadata into the actual type used by the rest of
/// the code.
impl Upload {
    // TODO: proper error handling of zero-length files array
    pub fn new_migrated(path: PathBuf, upload: UploadMetadata) -> Result<Self, ParseUploadError> {
        #[allow(unused_mut)]
        let mut current = upload;

        #[allow(clippy::never_loop)]
        loop {
            match current {
                UploadMetadata::V1(data) => break Upload::from_latest(path, data),
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
        path: PathBuf,
        upload: LatestUploadFormat,
    ) -> Result<Self, ParseUploadError> {
        let files = upload
            .files
            .into_iter()
            .map(|f| UploadFile {
                path: f.path,
                filename: f.filename,
                size: f.size,
            })
            .collect::<Vec<UploadFile>>();

        if files.is_empty() {
            return Err(ParseUploadError::ZeroFiles);
        }

        Ok(Self {
            path,
            total_size: upload.total_size,
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            // TODO: handle files.len() == 0
            files: if files.len() > 1 {
                UploadContents::Multiple(files)
            } else {
                UploadContents::Single(files[0].clone()) // TODO: don't clone
            },
        })
    }
}
