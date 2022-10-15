use std::{io, path::PathBuf};

use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use super::upload::Upload;
use crate::serde::UploadMetadata;

/// Make sure to call [`flush`] or else the metadata won't be saved!
///
/// [`flush`]: fn@Self::flush
///
/// some fields marked pub(super) so it can be constructed by [`super::file`]
// TODO: impl Drop so it can automatically flush() with RAII
#[derive(Debug)]
pub struct UploadHandle {
    /// the path to the upload directory
    pub(super) path: PathBuf,
    pub metadata: Upload,
    pub file: File,
    pub file_path: PathBuf,
    pub(super) metadata_file: File,
    pub(super) metadata_path: PathBuf,
}
#[derive(Debug, Error)]
pub enum SerializeMetadataError {
    #[error("error while serializing with serde_json")]
    Serialize(#[from] serde_json::Error),
    #[error("error writing metadata to file")]
    WriteMetadata(#[source] io::Error),

    #[error("error flushing metadata to disk")]
    FlushMetadata(#[source] io::Error),
    #[error("error flushing upload file to disk")]
    FlushFile(#[source] io::Error),
}
impl UploadHandle {
    pub async fn flush(mut self) -> Result<Upload, SerializeMetadataError> {
        self.metadata_file
            .write_all(
                // TODO: get rid of self.metadata.clone()
                serde_json::to_string(&UploadMetadata::from_upload(self.metadata.clone()))?
                    .as_bytes(),
            )
            .await
            .map_err(SerializeMetadataError::WriteMetadata)?;
        self.metadata_file
            .flush()
            .await
            .map_err(SerializeMetadataError::FlushMetadata)?;

        self.file
            .flush()
            .await
            .map_err(SerializeMetadataError::FlushFile)?;

        Ok(self.metadata)
    }
}
