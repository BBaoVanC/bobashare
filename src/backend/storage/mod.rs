//! Modules that handle storing uploaded files and serialized metadata.

use std::{path::PathBuf, future::Future};

use axum::{body::Bytes, extract::multipart::MultipartError};
use chrono::{prelude::*, Duration};
use futures_core::Stream;
use thiserror::Error;
use tokio::{fs, io};
use tracing::{event, instrument, Level};

use super::{Upload, UploadFile};

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
#[derive(Debug, Error)]
pub enum QueryUploadError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}
#[derive(Debug, Error)]
pub enum DeleteUploadError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}

#[derive(Debug)]
pub struct UploadRequestFile<'r> {
    filename: &'r str,
    mimetype: &'r str,
    contents: Box<dyn Stream<Item = Result<Bytes, MultipartError>>>,
}

impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    pub async fn new(path: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir(&path).await?;
        Ok(Self { path })
    }

    // TODO: use stream instead of Vec<u8>
    #[instrument]
    async fn create_upload(
        &self,
        url: String,
        files: Vec<UploadRequestFile<'_>>,
        expiry: Duration,
    ) -> Result<Upload, CreateUploadError> {
        if files.is_empty() {
            event!(Level::DEBUG, "cannot create upload with zero files");
            return Err(CreateUploadError::ZeroFiles);
        }

        let creation_date = Utc::now();
        let expiry_date = creation_date + expiry;
        let upload_root = self.path.join(&url);

        event!(Level::DEBUG, "creating directory to store upload");
        fs::create_dir(&upload_root)
            .await
            .map_err(|e| match e.kind() {
                io::ErrorKind::AlreadyExists => CreateUploadError::AlreadyExists,
                _ => CreateUploadError::IoError(e),
            })?; // TODO: make this statement less ugly, get rid of the match

        let mut upload = Upload {
            url,
            creation_date,
            expiry_date,
            files: Vec::new(),
            total_size: 0,
        };

        for (i, file) in files.into_iter().enumerate() {
            let name_on_disk = format!("{:0<4}", i); // sanitized name of the file on disk
            fs::write(upload_root.join(&name_on_disk), &file.contents).await?;
            let size = file.contents.len() as u64;
            upload.total_size += size;
            upload.files.push(UploadFile {
                path: PathBuf::from(name_on_disk),
                filename: String::from(file.filename),
                mimetype: String::from(file.mimetype),
                size,
            });
        }

        Ok(upload)
    }

    async fn check_exists(&self, url: String) -> Result<bool, QueryUploadError> {
        todo!()
    }

    async fn query_metadata(&self, url: String) -> Result<Upload, QueryUploadError> {
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
