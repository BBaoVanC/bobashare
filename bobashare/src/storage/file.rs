use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use thiserror::Error;
use tokio::{
    fs::{self},
    io,
};

use super::upload::Upload;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("the file {0} is not a directory")]
    NotADirectory(PathBuf),
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileBackend {
    pub path: PathBuf,
}
impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    pub async fn new(path: PathBuf) -> Result<Self, BackendError> {
        if let Err(e) = fs::create_dir(&path).await {
            if e.kind() != io::ErrorKind::AlreadyExists {
                // ignore AlreadyExists; propagate all other errors
                return Err(BackendError::from(e));
            }
        }
        if !fs::metadata(&path).await?.is_dir() {
            return Err(BackendError::NotADirectory(path));
        }

        Ok(Self { path })
    }
}

#[derive(Debug, Error)]
pub enum CreateUploadError {
    #[error("an upload with the requested name already exists")]
    AlreadyExists,
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}
impl FileBackend {
    pub async fn create_upload<S: AsRef<str>>(
        &self,
        url: S,
        expiry: Option<Duration>,
    ) -> Result<Upload, CreateUploadError> {
        let creation_date = Utc::now();
        let expiry_date = expiry.map(|e| creation_date + e);
        let upload_root = self.path.join(url.as_ref());

        fs::create_dir(&upload_root)
            .await
            .map_err(|e| match e.kind() {
                io::ErrorKind::AlreadyExists => CreateUploadError::AlreadyExists,
                _ => CreateUploadError::from(e),
            })?; // TODO: make this statement less ugly, get rid of the match

        Ok(Upload {
            path: upload_root,
            creation_date,
            expiry_date,
            files: Vec::new(),
            // total_size: 0,
            saved: false,
        })
    }
}
