//! Modules that handle storing uploaded files and serialized metadata.

use std::{path::PathBuf};

use chrono::{prelude::*, Duration};
use thiserror::Error;
use tokio::{fs, io};
use tracing::{event, instrument, Level};


pub mod metadata;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Upload {
    pub path: PathBuf,
    pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFile>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UploadFile {
    pub path: PathBuf,
    pub filename: String,
    pub mimetype: String,
    // only a hint
    pub size: u64,
}

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
        url: String,
        expiry: Option<Duration>,
    ) -> Result<Upload, CreateUploadError> {

        let creation_date = Utc::now();
        let expiry_date = expiry.map(|e| creation_date + e);
        let upload_root = self.path.join(&url);

        event!(Level::DEBUG, "creating directory to store upload");
        fs::create_dir(&upload_root)
            .await
            .map_err(|e| match e.kind() {
                io::ErrorKind::AlreadyExists => CreateUploadError::AlreadyExists,
                _ => CreateUploadError::IoError(e),
            })?; // TODO: make this statement less ugly, get rid of the match

        Ok(Upload {
            path: PathBuf::from(url),
            creation_date,
            expiry_date,
            files: Vec::new(),
            total_size: 0,
        })

        // for (i, file) in files.into_iter().enumerate() {
        //     let name_on_disk = format!("{:0<4}", i); // sanitized name of the file on disk
        //     fs::write(upload_root.join(&name_on_disk), &file.contents).await?;
        //     let size = file.contents.len() as u64;
        //     upload.total_size += size;
        //     upload.files.push(UploadFile {
        //         path: PathBuf::from(name_on_disk),
        //         filename: String::from(file.filename),
        //         mimetype: String::from(file.mimetype),
        //         size,
        //     });
        // }
    }
}

impl Upload {
    pub fn add_file_from_stream(&mut self, file: Vec<u8>) {

    }
}
