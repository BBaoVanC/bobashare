use std::path::{Path, PathBuf};

use chrono::prelude::*;
use thiserror::Error;
use tokio::{fs, fs::File, io, io::AsyncWriteExt};
use tracing::{event, instrument, Level};

use crate::serde::{IntoMetadataError, UploadMetadata};

#[derive(Debug, Clone)]
pub struct Upload {
    pub path: PathBuf,
    // pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFile>,
}
#[derive(Debug, Clone)]
pub struct UploadFile {
    pub path: PathBuf,
    pub filename: String,
    pub mimetype: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct UploadHandle {
    pub metadata: Upload,
    data_file: File,
    saved: bool,
}
impl UploadHandle {
    pub async fn new(metadata: Upload) -> Result<Self, io::Error> {
        let data_file = File::create(metadata.path.join("metadata.json")).await?;
        Ok(Self {
            metadata,
            data_file,
            saved: false,
        })
    }
}

#[derive(Debug, Error)]
pub enum SerializeMetadataError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
    #[error("error converting Upload to UploadMetadata")]
    FromMetadataError(#[from] IntoMetadataError),
    #[error("error while serializing with serde_json")]
    SerdeError(#[from] serde_json::Error),
}
impl UploadHandle {
    pub async fn save(mut self) -> Result<(), SerializeMetadataError> {
        self.data_file
            .write_all(
                // TODO: get rid of self.metadata.clone()
                serde_json::to_string(&UploadMetadata::from_upload(self.metadata.clone()).await?)?
                    .as_bytes(),
            )
            .await?;

        self.saved = true;
        Ok(())
    }
}
impl Drop for UploadHandle {
    #[instrument]
    fn drop(&mut self) {
        if !self.saved {
            event!(
                Level::ERROR,
                "An UploadHandle was dropped without using the save() method! \
                The metadata has not been saved!"
            );
        } else {
            event!(
                Level::TRACE,
                "UploadHandle was dropped after properly calling save()"
            );
        }
    }
}

#[derive(Debug)]
pub struct UploadFileHandle<'h> {
    pub metadata: UploadFile,
    pub file: File,
    files_vec: &'h mut Vec<UploadFile>,
}
impl<'h> UploadFileHandle<'_> {
    pub fn new(
        metadata: UploadFile,
        file: File,
        files_vec: &'h mut Vec<UploadFile>,
    ) -> UploadFileHandle<'h> {
        UploadFileHandle {
            metadata,
            file,
            files_vec,
        }
    }
}
impl Drop for UploadFileHandle<'_> {
    /// Automatically add file to the [`Upload`] when the handle is dropped.
    fn drop(&mut self) {
        self.files_vec.push(self.metadata.clone());
    }
}

#[derive(Debug, Error)]
pub enum CreateFileError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}
impl Upload {
    pub async fn create_file<P: AsRef<Path>, S: AsRef<str>>(
        &mut self,
        path: P,
        filename: S,
        mimetype: S,
    ) -> Result<UploadFileHandle, CreateFileError> {
        let file = File::create(self.path.join(path.as_ref())).await?;

        let metadata = UploadFile {
            path: path.as_ref().to_path_buf(),
            filename: filename.as_ref().to_string(),
            mimetype: mimetype.as_ref().to_string(),
        };
        let handle = UploadFileHandle::new(metadata, file, &mut self.files);
        Ok(handle)
    }
}
