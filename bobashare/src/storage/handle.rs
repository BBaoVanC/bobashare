use std::{
    ffi::{OsStr, OsString},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};

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
// TODO: impl Drop so it can automatically flush() with RAII
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
    #[error("the file already exists: {}",.0.to_string_lossy())]
    AlreadyExists(OsString),
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}
impl UploadHandle<'_> {
    pub async fn delete(self) -> Result<(), io::Error> {
        todo!()
    }
    pub async fn delete_file(&mut self, handle: UploadFileHandle<'_>) -> Result<(), io::Error> {
        fs::remove_file(&handle.full_path).await?;
        todo!("remove from Vec");
        Ok(())
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

    pub async fn create_file<P: AsRef<OsStr>, S: AsRef<str>>(
        &mut self,
        path: P,
        filename: S,
        mimetype: S,
    ) -> Result<UploadFileHandle, CreateFileError> {
        let path = path.as_ref();

        let full_path = self.metadata.path.join(path);

        let metadata = UploadFile {
            path: OsString::from(path),
            filename: filename.as_ref().to_string(),
            mimetype: mimetype.as_ref().to_string(),
        };

        let path_owned = OsString::from(path);
        if self.metadata.files.contains_key(path) {
            return Err(CreateFileError::AlreadyExists(path_owned));
        }
        self.metadata.files.insert(path_owned, metadata);

        let file = File::create(&full_path).await?;

        Ok(UploadFileHandle {
            metadata: self.metadata.files.get(path).unwrap(),
            // metadata: self.metadata.files.last().unwrap(),
            file,
            full_path,
        })
    }

    pub async fn read_file() {
        todo!()
    }

    pub async fn open_file() {
        todo!()
    }
}
// impl Drop for UploadHandle<'_> {
//     /// Only for logging
//     #[instrument]
//     fn drop(&mut self) {
//         event!(Level::TRACE, "UploadHandle was dropped");
//     }
// }

#[derive(Debug)]
pub struct UploadFileHandle<'h> {
    pub metadata: &'h UploadFile,
    pub file: File,
    full_path: PathBuf,
}
// impl Drop for UploadFileHandle<'_> {
//     /// Only for logging
//     #[instrument]
//     fn drop(&mut self) {
//         event!(Level::TRACE, "UploadFileHandle was dropped");
//     }
// }
