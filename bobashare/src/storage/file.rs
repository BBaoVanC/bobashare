use std::{collections::HashMap, path::PathBuf};

use chrono::{prelude::*, Duration};
use thiserror::Error;
use tokio::{fs, io};
use tracing::instrument;

use crate::generate_randomized_name;

use super::upload::Upload;

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
        url: S,
        expiry: Option<Duration>,
    ) -> Result<Upload, CreateUploadError> {
        let creation_date = Utc::now();
        let expiry_date = expiry.map(|e| creation_date + e);
        let path = self.path.join(url.as_ref());

        fs::create_dir(&path).await.map_err(|e| match e.kind() {
            io::ErrorKind::AlreadyExists => CreateUploadError::AlreadyExists,
            _ => CreateUploadError::from(e),
        })?; // TODO: make this statement less ugly, get rid of the match

        // TODO: this probably isnt needed, could cause confusing errors
        // let path = fs::canonicalize(path).await?;

        Ok(Upload {
            path,
            creation_date,
            expiry_date,
            files: HashMap::new(),
        })
    }

    pub async fn create_upload_random_name(&self, length: usize, expiry: Option<Duration>) -> Result<Upload, CreateUploadError> {
        let url = generate_randomized_name(length);
        self.create_upload(url, expiry).await
    }

    // TODO: maybe create_upload_with_capacity
}
