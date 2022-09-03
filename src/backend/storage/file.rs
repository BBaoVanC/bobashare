//! A backend that stores as files

use std::path::PathBuf;

use async_trait::async_trait;
use chrono::{prelude::*, Duration};
use tokio::{fs, io};
use tracing::{event, instrument, Level};

use super::{CreateUploadError, StorageBackend};
use crate::backend::{Upload, UploadFile};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileBackend {
    pub path: PathBuf,
}
impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    pub async fn new(path: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir(&path).await?;
        Ok(Self { path })
    }
}

#[async_trait]
impl StorageBackend for FileBackend {
    #[instrument]
    async fn create_upload(&self, url: String, files: Vec<UploadFile>, expiry: Duration) -> Result<Upload, CreateUploadError> {
        if files.is_empty() {
            event!(Level::DEBUG, "cannot create upload with zero files");
            return Err(CreateUploadError::ZeroFiles);
        }

        let creation_date = Utc::now();
        let expiry_date = creation_date + expiry;
        let path = self.path.join(url);

        event!(Level::DEBUG, "creating directory to store upload");
        fs::create_dir(&path).await.map_err(|e| CreateUploadError::AlreadyExists)?;

        todo!()
    }
}
