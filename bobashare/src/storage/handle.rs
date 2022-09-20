use std::{io, path::Path};

use relative_path::{FromPathError, RelativePathBuf};
use thiserror::Error;
use tokio::{fs::{File, self}, io::AsyncWriteExt};
use tracing::{event, Level, instrument};

use crate::serde::{UploadMetadata, IntoMetadataError};

use super::upload::{UploadFile, Upload};

/// Make sure to call [`flush`] or else the metadata won't be saved!
/// 
/// [`flush`]: fn@Self::flush
// TODO: impl Drop so it can automatically flush with RAII
#[derive(Debug)]
pub struct UploadHandle<'h> {
    pub metadata: &'h mut Upload,
    data_file: File,
    saved: bool,
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
#[derive(Debug, Error)]
pub enum CreateFileError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
    #[error("error converting path to a relative path")]
    RelativePathError(#[from] FromPathError),
}
impl UploadHandle<'_> {
    pub async fn delete(self) -> Result<(), io::Error> {
        todo!()
    }

    #[instrument]
    pub async fn flush(mut self) -> Result<(), SerializeMetadataError> {
        event!(Level::TRACE, "UploadHandle.save() called");
        self.data_file
            .write_all(
                // TODO: get rid of self.metadata.clone()
                serde_json::to_string(&UploadMetadata::from_upload(self.metadata.clone()))?
                    .as_bytes(),
            )
            .await?;

        self.saved = true;
        Ok(())
    }

    pub async fn create_file<P: AsRef<Path>, S: AsRef<str>>(
        &mut self,
        path: P,
        filename: S,
        mimetype: S,
    ) -> Result<UploadFileHandle, CreateFileError> {
        let file = File::create(self.metadata.path.join(path.as_ref())).await?;

        let metadata = UploadFile {
            path: RelativePathBuf::from_path(path)?,
            filename: filename.as_ref().to_string(),
            mimetype: mimetype.as_ref().to_string(),
        };

        self.metadata.files.push(metadata);

        let handle = UploadFileHandle::new(self.metadata.files.last().unwrap(), file);
        // let handle = UploadFileHandle::new(metadata, file, &mut self.files);
        Ok(handle)
    }

    pub async fn read_file<'f>(&'f self, metadata: &'f UploadFile) -> Result<UploadFileHandle, io::Error> {
        let file = File::open(metadata.path.to_path(&self.metadata.path)).await?;

        Ok(UploadFileHandle {
            file,
            metadata,
        })
    }
}
impl Drop for UploadHandle<'_> {
    /// Only for logging
    #[instrument]
    fn drop(&mut self) {
        event!(
            Level::TRACE,
            "UploadHandle was dropped"
        );
    }
}

#[derive(Debug)]
pub struct UploadFileHandle<'h> {
    pub metadata: &'h UploadFile,
    pub file: File,
}
impl<'h> UploadFileHandle<'_> {
    pub fn new(
        metadata: &'h UploadFile,
        file: File,
    ) -> UploadFileHandle<'h> {
        UploadFileHandle {
            metadata,
            file,
        }
    }

    pub fn delete(self) -> Result<(), io::Error> {
        todo!()
    }
}
