use std::{io, path::PathBuf};

use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt};

use super::upload::Upload;
use crate::serde::{IntoMetadataError, UploadMetadata};

/// Make sure to call [`flush`] or else the metadata won't be saved!
///
/// [`flush`]: fn@Self::flush
// TODO: impl Drop so it can automatically flush() with RAII
#[derive(Debug)]
pub struct UploadHandle {
    pub metadata: Upload,
    pub file: File,
    pub file_path: PathBuf,
    // pub(super) so it can be constructed by [`super::file`]
    pub(super) metadata_file: File,
}
#[derive(Debug, Error)]
pub enum SerializeMetadataError {
    #[error("error while doing i/o: {0}")]
    Io(#[from] io::Error),
    #[error("error converting Upload to UploadMetadata")]
    FromMetadata(#[from] IntoMetadataError),
    #[error("error while serializing with serde_json")]
    Serde(#[from] serde_json::Error),
}
impl UploadHandle {
    pub async fn flush(mut self) -> Result<Upload, SerializeMetadataError> {
        self.metadata_file
            .write_all(
                // TODO: get rid of self.metadata.clone()
                serde_json::to_string(&UploadMetadata::from_upload(self.metadata.clone()))?
                    .as_bytes(),
            )
            .await?;
        self.metadata_file.sync_all().await?;

        self.file.sync_all().await?;

        Ok(self.metadata)
    }
}
impl UploadHandle {
    pub async fn delete(self) -> Result<(), io::Error> {
        todo!()
    }
}
