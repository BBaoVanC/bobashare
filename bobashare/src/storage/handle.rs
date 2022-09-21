use std::{
    io,
    path::{Path, PathBuf},
};

use relative_path::{FromPathError, RelativePathBuf};
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tracing::{event, instrument, Level};

use super::upload::{Upload, UploadFile};
use crate::serde::{IntoMetadataError, UploadMetadata};

/// Make sure to call [`flush`] or else the metadata won't be saved!
///
/// [`flush`]: fn@Self::flush
// TODO: impl Drop so it can automatically flush with RAII
#[derive(Debug)]
pub struct UploadHandle<'h> {
    pub metadata: &'h mut Upload,
    data_file: File,
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
        self.data_file
            .write_all(
                // TODO: get rid of self.metadata.clone()
                serde_json::to_string(&UploadMetadata::from_upload(self.metadata.clone()))?
                    .as_bytes(),
            )
            .await?;
        self.data_file.flush().await?;
        Ok(())
    }

    pub async fn create_file<P: AsRef<Path>, S: AsRef<str>>(
        &mut self,
        path: P,
        filename: S,
        mimetype: S,
    ) -> Result<UploadFileHandle, CreateFileError> {
        let full_path = self.metadata.path.join(path.as_ref());
        let file = File::create(&full_path).await?;

        let metadata = UploadFile {
            path: RelativePathBuf::from_path(path)?,
            filename: filename.as_ref().to_string(),
            mimetype: mimetype.as_ref().to_string(),
        };

        self.metadata.files.push(metadata);

        Ok(UploadFileHandle {
            metadata: self.metadata.files.last().unwrap(),
            file,
            full_path,
        })
    }

    pub async fn read_file<'f>(
        &'f self,
        metadata: &'f UploadFile,
    ) -> Result<UploadFileHandle, io::Error> {
        let full_path = metadata.path.to_path(&self.metadata.path);
        let file = File::open(&full_path).await?;

        // Ok(UploadFileHandle::new(metadata, file, &self.metadata.path))
        Ok(UploadFileHandle {
            metadata,
            file,
            full_path,
        })
    }

    pub async fn open_file<'f>(
        &'f self,
        metadata: &'f UploadFile,
        options: fs::OpenOptions,
    ) -> Result<UploadFileHandle, io::Error> {
        let full_path = metadata.path.to_path(&self.metadata.path);
        let file = options.open(&full_path).await?;

        // Ok(UploadFileHandle::new(metadata, file, &self.metadata.path))
        Ok(UploadFileHandle {
            metadata,
            file,
            full_path,
        })
    }
}
impl Drop for UploadHandle<'_> {
    /// Only for logging
    #[instrument]
    fn drop(&mut self) {
        event!(Level::TRACE, "UploadHandle was dropped");
    }
}

#[derive(Debug)]
pub struct UploadFileHandle<'h> {
    pub metadata: &'h UploadFile,
    pub file: File,
    full_path: PathBuf,
}
impl<'h> UploadFileHandle<'_> {
    // fn new<P: AsRef<Path>>(metadata: &'h UploadFile, file: File, root: P) ->
    // UploadFileHandle<'h> {     UploadFileHandle { metadata, file, full_path:
    // metadata.path.to_path(root) } }

    pub fn delete(self) -> Result<(), io::Error> {
        todo!()
    }
}
impl Drop for UploadFileHandle<'_> {
    /// Only for logging
    #[instrument]
    fn drop(&mut self) {
        event!(Level::TRACE, "UploadFileHandle was dropped");
    }
}
