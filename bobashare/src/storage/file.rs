use std::{path::PathBuf};

use chrono::{prelude::*, Duration};
use mime::Mime;
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io,
};

use super::{handle::UploadHandle, upload::Upload};

#[derive(Debug, Error)]
pub enum NewBackendError {
    #[error("the file {0} is not a directory")]
    NotADirectory(PathBuf),
    #[error("error while doing i/o: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileBackend {
    pub path: PathBuf,
}
impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    // #[instrument]
    pub async fn new(path: PathBuf) -> Result<Self, NewBackendError> {
        if let Err(e) = fs::create_dir(&path).await {
            // ignore AlreadyExists; propagate all other errors
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(NewBackendError::from(e));
            }
        }

        let path = fs::canonicalize(path).await?;
        if !fs::metadata(&path).await?.is_dir() {
            return Err(NewBackendError::NotADirectory(path));
        }

        Ok(Self { path })
    }
}

#[derive(Debug, Error)]
pub enum CreateUploadError {
    #[error("an upload with the requested name already exists")]
    AlreadyExists,
    #[error(transparent)]
    Io(#[from] io::Error),
}
impl FileBackend {
    pub async fn create_upload<S: AsRef<str>>(
        &self,
        id: S,
        filename: S,
        mimetype: Mime,
        size: Option<u64>,
        expiry: Option<Duration>,
    ) -> Result<UploadHandle, CreateUploadError> {
        let creation_date = Utc::now();
        let expiry_date = expiry.map(|e| creation_date + e);
        let path = self.path.join(id.as_ref());

        fs::create_dir(&path).await.map_err(|e| match e.kind() {
            io::ErrorKind::AlreadyExists => CreateUploadError::AlreadyExists,
            _ => CreateUploadError::from(e),
        })?; // TODO: make this statement less ugly, get rid of the match

        let metadata_file = File::create(path.join("metadata.json")).await?;
        let file = File::create(path.join(id.as_ref())).await?;

        Ok(UploadHandle {
            metadata: Upload {
                id: String::from(id.as_ref()),
                filename: String::from(filename.as_ref()),
                mimetype,
                size,
                creation_date,
                expiry_date,
            },
            metadata_file,
            file,
        })
    }
}
