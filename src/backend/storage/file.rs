//! A backend that stores as files

use std::path::PathBuf;

use async_trait::async_trait;
use chrono::{prelude::*, Duration};
use tokio::{fs, io};
use tracing::{event, instrument, Level};

use crate::backend::{
    generate_randomized_name, CreateUploadError, StorageBackend, Upload, UploadFile,
};

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
    async fn create_upload(
        &self,
        files: Vec<UploadFile>,
        expiry: Duration,
    ) -> Result<(), CreateUploadError> {
        if files.is_empty() {
            event!(Level::DEBUG, "cannot upload zero files");
            return Err(CreateUploadError::ZeroFiles);
        }

        let now = Utc::now();
        let expiry = now + expiry;
        let path = self.path.join(generate_randomized_name());

        loop {
            match fs::create_dir(&path).await {
                Ok(()) => break,
                Err(e) => event!(
                    Level::TRACE,
                    "error while creating directory for upload, retrying: {}",
                    e
                ),
            }
        }

        todo!()
    }
}
