use std::{
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
};

use super::upload::{Upload, UploadFile};
use crate::serde::{IntoMetadataError, UploadMetadata};

/// Make sure to call [`flush`] or else the metadata won't be saved!
///
/// [`flush`]: fn@Self::flush
// TODO: impl Drop so it can automatically flush() with RAII
#[derive(Debug)]
pub struct UploadHandle {
    pub metadata: Upload,
    // pub(super) so it can be accessed by [`super::file`]
    pub(super) data_file: File,
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
        self.data_file
            .write_all(
                // TODO: get rid of self.metadata.clone()
                serde_json::to_string(&UploadMetadata::from_upload(self.metadata.clone()))?
                    .as_bytes(),
            )
            .await?;
        self.data_file.flush().await?;
        Ok(self.metadata)
    }
}
#[derive(Debug, Error)]
pub enum CreateFileError {
    #[error("the file already exists")]
    AlreadyExists,
    #[error("error while doing i/o: {0}")]
    Io(#[from] io::Error),
}
impl UploadHandle {
    pub async fn delete(self) -> Result<(), io::Error> {
        todo!()
    }
    pub async fn delete_file(&mut self) -> Result<(), io::Error> {
        todo!()
    }

    pub async fn create_file<S: AsRef<str>>(
        &mut self,
        url: S,
        filename: S,
        mimetype: S,
    ) -> Result<UploadFileHandle, CreateFileError> {
        let url = url.as_ref();

        let metadata = UploadFile {
            filename: filename.as_ref().to_string(),
            mimetype: mimetype.as_ref().to_string(),
        };

        if self.metadata.files.contains_key(url) {
            return Err(CreateFileError::AlreadyExists);
        }
        self.metadata.files.insert(String::from(url), metadata);

        let full_path = Path::new(&self.metadata.url).join(url);
        let file = File::create(&full_path).await?;

        Ok(UploadFileHandle {
            // TODO: assert this unwrap can never fail
            metadata: self.metadata.files.get(url).unwrap(),
            full_path,
            file,
        })
    }
}
#[derive(Debug, Error)]
pub enum OpenFileError {
    #[error("the file is not listed in the Upload metadata")]
    NotFound,
    #[error(transparent)]
    Io(#[from] io::Error),
}
impl UploadHandle {
    pub async fn read_file<S: AsRef<str>>(
        &self,
        url: S,
    ) -> Result<UploadFileHandle, OpenFileError> {
        self.open_file(url, OpenOptions::new().read(true)).await
    }

    async fn open_file<S: AsRef<str>>(
        &self,
        url: S,
        options: &OpenOptions,
    ) -> Result<UploadFileHandle, OpenFileError> {
        let url = url.as_ref();

        let metadata = self
            .metadata
            .files
            .get(url)
            .ok_or(OpenFileError::NotFound)?;
        let full_path = Path::new(&self.metadata.url).join(url);
        let file = options.open(&full_path).await?;

        Ok(UploadFileHandle {
            metadata,
            full_path,
            file,
        })
    }
}

// TODO: impl Drop
#[derive(Debug)]
pub struct UploadFileHandle<'h> {
    pub metadata: &'h UploadFile,
    pub file: File,
    full_path: PathBuf,
}
