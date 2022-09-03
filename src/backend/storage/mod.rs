//! Modules that handle storing uploaded files and serialized metadata.

use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use thiserror::Error;
use tokio::{io, fs};
use tracing::{instrument, event, Level};

use super::{serde::UploadMetadata, Upload, UploadFile};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileBackend {
    pub path: PathBuf,
}

#[derive(Debug, Error)]
pub enum CreateUploadError {
    #[error("the list of files to upload was empty")]
    ZeroFiles,
    #[error("an upload with the requested name already exists")]
    AlreadyExists,
}
#[derive(Debug, Error)]
pub enum QueryUploadError {
    #[error("error while doing i/o")]
    IoError(#[from] std::io::Error),
}
#[derive(Debug, Error)]
pub enum DeleteUploadError {
    #[error("error while doing i/o")]
    IoError(#[from] std::io::Error),
}

impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    pub async fn new(path: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir(&path).await?;
        Ok(Self { path })
    }

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

    async fn check_exists(&self, url: String) -> Result<bool, QueryUploadError> {
        todo!()
    }

    async fn query_metadata(&self, url: String) -> Result<UploadMetadata, QueryUploadError> {
        todo!()
    }

    async fn stream_file(
        &self,
        url: String,
        file: String,
    ) -> Result<io::BufReader<u8>, QueryUploadError> {
        todo!()
    }

    async fn delete_upload(&self, url: String) -> Result<(), DeleteUploadError> {
        todo!()
    }
}
