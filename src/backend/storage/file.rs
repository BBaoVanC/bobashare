//! A backend that stores as files

use std::{fs, io, path::{PathBuf, Path}};

use chrono::Duration;
use thiserror::Error;

use crate::backend::{UploadFile, Upload, generate_randomized_name};

use chrono::prelude::*;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CreateUploadError {
    #[error("the list of files to upload was empty")]
    ZeroFiles,
}

pub struct FileBackend {
    pub path: PathBuf,
}
impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir(&path)?;
        Ok(Self { path })
    }

    pub async fn create_upload(&self, files: Vec<UploadFile>, expiry: Duration) -> Result<Upload, CreateUploadError> {
        if files.is_empty() {
            return Err(CreateUploadError::ZeroFiles);
        }

        let now = Utc::now();
        let expiry = now + expiry;

        todo!()
    }
}
