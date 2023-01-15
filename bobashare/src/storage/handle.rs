//! Methods to create a handle (RAII guard) to interact with an upload stored on
//! disk.
//!
//! NOTE: Currently you must call [`UploadHandle::flush`] since it can't do
//! that automatically yet without an async [`Drop`] impl.

use std::{io, path::PathBuf};

use displaydoc::Display;
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use super::upload::Upload;
use crate::serde::UploadMetadata;

/// Make sure to call [`Self::flush`] or else the metadata won't be saved!
// TODO: impl Drop so it can automatically flush() with RAII
#[derive(Debug)]
pub struct UploadHandle {
    /// path to the upload directory
    pub path: PathBuf,
    /// info about the upload, see [`Upload`]
    pub metadata: Upload,
    /// reference to the open uploaded file
    pub file: File,
    /// path of the uploaded file
    pub file_path: PathBuf,
    // marked pub(super) so it can be constructed by [`super::file`] methods
    pub(super) metadata_file: File,
    pub(super) lock_path: PathBuf,
}
/// Errors when flushing the upload metadata to disk
#[derive(Debug, Error, Display)]
pub enum FlushUploadError {
    /// error while serializing with serde_json
    Serialize(#[from] serde_json::Error),
    /// error writing metadata to file
    WriteMetadata(#[source] io::Error),

    /// error flushing metadata to disk
    FlushMetadata(#[source] io::Error),
    /// error flushing upload file to disk
    FlushFile(#[source] io::Error),

    /// error removing lock file
    RemoveLock(#[source] io::Error),
}
impl UploadHandle {
    /// Consume the handle, gracefully close the uploaded file, and flush the
    /// metadata to disk.
    pub async fn flush(mut self) -> Result<Upload, FlushUploadError> {
        self.metadata_file
            .write_all(
                // TODO: get rid of self.metadata.clone()
                serde_json::to_string(&UploadMetadata::from_upload(self.metadata.clone()))?
                    .as_bytes(),
            )
            .await
            .map_err(FlushUploadError::WriteMetadata)?;
        self.metadata_file
            .flush()
            .await
            .map_err(FlushUploadError::FlushMetadata)?;

        self.file
            .flush()
            .await
            .map_err(FlushUploadError::FlushFile)?;

        fs::remove_file(&self.lock_path)
            .await
            .map_err(FlushUploadError::RemoveLock)?;

        Ok(self.metadata)
    }
}
