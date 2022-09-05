use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use relative_path::RelativePathBuf;
use thiserror::Error;
use tokio::{fs, io};
use tracing::{event, instrument, Level};

use super::upload::Upload;

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
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}

impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    pub async fn new(path: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir(&path).await?;
        Ok(Self { path })
    }

    // TODO: use stream instead of Vec<u8>
    #[instrument]
    pub async fn create_upload(
        &self,
        url: &str,
        expiry: Option<Duration>,
    ) -> Result<Upload, CreateUploadError> {
        let creation_date = Utc::now();
        let expiry_date = expiry.map(|e| creation_date + e);
        let upload_root = self.path.join(url);

        event!(Level::DEBUG, "creating directory to store upload");
        fs::create_dir(&upload_root)
            .await
            .map_err(|e| match e.kind() {
                io::ErrorKind::AlreadyExists => CreateUploadError::AlreadyExists,
                _ => CreateUploadError::IoError(e),
            })?; // TODO: make this statement less ugly, get rid of the match

        Ok(Upload {
            path: RelativePathBuf::from(url),
            creation_date,
            expiry_date,
            files: Vec::new(),
            total_size: 0,
        })
    }
}
