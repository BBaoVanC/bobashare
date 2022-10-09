use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use mime::Mime;
use thiserror::Error;
use tokio::{
    fs::{self, File, OpenOptions},
    io::{self, AsyncReadExt},
};

use super::{handle::UploadHandle, upload::Upload};
use crate::serde::{MigrateError, UploadMetadata};

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
        let file_path = path.join(id.as_ref());
        let file = File::create(&file_path).await?;

        Ok(UploadHandle {
            metadata: Upload {
                id: String::from(id.as_ref()),
                filename: String::from(filename.as_ref()),
                mimetype,
                creation_date,
                expiry_date,
            },
            file,
            file_path,
            metadata_file,
        })
    }
}

#[derive(Debug, Error)]
pub enum OpenUploadError {
    #[error("the upload was not found")]
    NotFound(io::Error),
    #[error("error while reading metadata file")]
    ReadMetadata(io::Error),

    #[error("error while opening upload file")]
    OpenFile(io::Error),

    #[error("error deserializing upload metadata")]
    DeserializeMetadata(#[from] serde_json::Error),
    #[error("error while migrating upload metadata to latest version")]
    MigrateMetadata(#[from] MigrateError),
}
impl FileBackend {
    pub async fn open_upload<S: AsRef<str>>(
        &self,
        id: S,
        write: bool,
    ) -> Result<UploadHandle, OpenUploadError> {
        let path = self.path.join(id.as_ref());
        let mut open_options = OpenOptions::new();
        open_options.read(true).create(false).write(write);

        let file_path = path.join(id.as_ref());
        let file = open_options
            .open(&file_path)
            .await
            .map_err(OpenUploadError::OpenFile)?;

        let metadata_path = path.join("metadata.json");
        let mut metadata_file = open_options
            .open(metadata_path)
            .await
            .map_err(OpenUploadError::NotFound)?;

        let mut metadata = String::new();
        metadata_file
            .read_to_string(&mut metadata)
            .await
            .map_err(OpenUploadError::ReadMetadata)?;
        let metadata = UploadMetadata::into_migrated_upload(
            id.as_ref().to_string(),
            serde_json::from_str(&metadata)?,
        )?;

        Ok(UploadHandle {
            metadata,
            metadata_file,
            file,
            file_path,
        })
    }
}
