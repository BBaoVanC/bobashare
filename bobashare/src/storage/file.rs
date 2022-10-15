use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use mime::Mime;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use thiserror::Error;
use tokio::{
    fs::{self, OpenOptions},
    io::{self, AsyncReadExt},
};

use super::{handle::UploadHandle, upload::Upload};
use crate::serde::{MigrateError, UploadMetadata};

#[derive(Debug, Error)]
pub enum NewBackendError {
    #[error("the file {0} is not a directory")]
    NotADirectory(PathBuf),
    #[error("error creating directory for file backend")]
    CreateDirectory(#[source] io::Error),
    #[error("error checking if backend path is directory")]
    ReadMetadata(#[source] io::Error),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileBackend {
    pub path: PathBuf,
}
impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    pub async fn new(path: PathBuf) -> Result<Self, NewBackendError> {
        if let Err(e) = fs::create_dir(&path).await {
            // ignore AlreadyExists; propagate all other errors
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(NewBackendError::CreateDirectory(e));
            }
        }

        if !fs::metadata(&path)
            .await
            .map_err(NewBackendError::ReadMetadata)?
            .is_dir()
        {
            return Err(NewBackendError::NotADirectory(path));
        }

        // this should not fail because we already verified that the path exists
        let path = fs::canonicalize(path).await.unwrap();

        Ok(Self { path })
    }

    fn get_upload_path<S: AsRef<str>>(&self, id: S) -> PathBuf {
        self.path.join(id.as_ref())
    }
    fn get_metadata_path<S: AsRef<str>>(&self, id: S) -> PathBuf {
        self.get_upload_path(id.as_ref()).join("metadata.json")
    }
    fn get_upload_file_path<S: AsRef<str>>(&self, id: S) -> PathBuf {
        self.get_upload_path(id.as_ref()).join(id.as_ref())
    }
}

#[derive(Debug, Error)]
pub enum CreateUploadError {
    #[error("an upload with the requested name already exists")]
    AlreadyExists,
    #[error("error creating parent directory for the upload")]
    CreateDirectory(#[source] io::Error),
    #[error("error creating metadata file")]
    CreateMetadataFile(#[source] io::Error),
    #[error("error creating file for upload contents")]
    CreateUploadFile(#[source] io::Error),
}
impl FileBackend {
    pub async fn create_upload<S: AsRef<str>>(
        &self,
        id: S,
        filename: S,
        mimetype: Mime,
        expiry: Option<Duration>,
        delete_key: Option<String>,
    ) -> Result<UploadHandle, CreateUploadError> {
        let creation_date = Utc::now();
        let expiry_date = expiry.map(|e| creation_date + e);
        let path = self.get_upload_path(id.as_ref());

        fs::create_dir(&path).await.map_err(|e| match e.kind() {
            io::ErrorKind::AlreadyExists => CreateUploadError::AlreadyExists,
            _ => CreateUploadError::CreateDirectory(e),
        })?;

        let metadata_path = self.get_metadata_path(id.as_ref());
        let metadata_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&metadata_path)
            .await
            .map_err(CreateUploadError::CreateMetadataFile)?;
        let file_path = self.get_upload_file_path(id.as_ref());
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&file_path)
            .await
            .map_err(CreateUploadError::CreateUploadFile)?;

        Ok(UploadHandle {
            path,
            metadata: Upload {
                id: String::from(id.as_ref()),
                filename: String::from(filename.as_ref()),
                mimetype,
                creation_date,
                expiry_date,
                delete_key: delete_key
                    .unwrap_or_else(|| Alphanumeric.sample_string(&mut thread_rng(), 32)),
            },
            file,
            file_path,
            metadata_file,
            metadata_path,
        })
    }
}

#[derive(Debug, Error)]
pub enum OpenUploadError {
    #[error("the upload was not found")]
    NotFound(#[source] io::Error),

    #[error("error while reading metadata file")]
    ReadMetadata(#[source] io::Error),
    #[error("error while opening upload file")]
    OpenFile(#[source] io::Error),

    #[error("error deserializing upload metadata")]
    DeserializeMetadata(#[from] serde_json::Error),
    #[error("error while migrating upload metadata to latest version")]
    MigrateMetadata(#[from] MigrateError),
}
impl FileBackend {
    pub async fn read_upload_metadata<S: AsRef<str>>(
        &self,
        id: S,
    ) -> Result<Upload, OpenUploadError> {
        let metadata_path = self.get_metadata_path(id.as_ref());
        let mut metadata_file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
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
        Ok(metadata)
    }
    // TODO: some method to only read upload metadata instead of needing to grab an
    // UploadHandle with write disabled
    /// does not check if the upload is expired, do that yourself
    pub async fn open_upload<S: AsRef<str>>(
        &self,
        id: S,
        write: bool,
    ) -> Result<UploadHandle, OpenUploadError> {
        let path = self.get_upload_path(id.as_ref());
        let mut open_options = OpenOptions::new();
        open_options.read(true).create(false).write(write);

        let metadata_path = self.get_metadata_path(id.as_ref());
        let mut metadata_file = open_options
            .open(&metadata_path)
            .await
            .map_err(OpenUploadError::NotFound)?;

        let file_path = self.get_upload_file_path(id.as_ref());
        let file = open_options
            .open(&file_path)
            .await
            .map_err(OpenUploadError::OpenFile)?;

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
            path,
            metadata,
            metadata_file,
            metadata_path,
            file,
            file_path,
        })
    }
}

#[derive(Debug, Error)]
pub enum DeleteUploadError {
    #[error("an upload at the specified id was not found")]
    NotFound,

    #[error("error deleting upload file")]
    DeleteFile(#[source] io::Error),
    #[error("error deleting metadata file")]
    DeleteMetadata(#[source] io::Error),
    #[error("error deleting upload directory")]
    DeleteDirectory(#[source] io::Error),
}
impl FileBackend {
    pub async fn delete_upload<S: AsRef<str>>(&self, id: S) -> Result<(), DeleteUploadError> {
        let path = self.get_upload_path(id.as_ref());
        if !path.is_dir() {
            return Err(DeleteUploadError::NotFound);
        }

        let metadata_path = self.get_metadata_path(id.as_ref());
        let file_path = self.get_upload_file_path(id.as_ref());
        fs::remove_file(file_path)
            .await
            .map_err(DeleteUploadError::DeleteFile)?;
        fs::remove_file(metadata_path)
            .await
            .map_err(DeleteUploadError::DeleteMetadata)?;
        fs::remove_dir(path)
            .await
            .map_err(DeleteUploadError::DeleteDirectory)?;

        Ok(())
    }
}
