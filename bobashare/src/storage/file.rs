//! A backend where uploads are stored as files on disk

use std::path::{PathBuf, Path};

use chrono::{prelude::*, Duration};
use displaydoc::Display;
use mime::Mime;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use thiserror::Error;
use tokio::{
    fs::{self, OpenOptions},
    io::{self, AsyncReadExt, BufReader},
};

use super::{handle::UploadHandle, upload::Upload};
use crate::serde::{MigrateError, UploadMetadata};

/// Errors when creating a new [`FileBackend`]
#[derive(Debug, Error, Display)]
pub enum NewBackendError {
    /// the file `{0}` is not a directory
    NotADirectory(PathBuf),
    /// error creating directory for file backend
    CreateDirectory(#[source] io::Error),
    /// error checking if backend path is directory
    ReadMetadata(#[source] io::Error),
}

/// A directory on disk which is used to store uploads
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileBackend {
    /// path of the directory containing all uploads
    pub path: PathBuf,
}
impl FileBackend {
    /// Construct a file backend, creating the directory if it doesn't exist.
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

    /// Get the path of the directory containing the upload
    fn get_upload_path<S: AsRef<str>>(&self, id: S) -> PathBuf {
        self.path.join(id.as_ref())
    }
    /// Get the path to the `metadata.json` of the upload
    fn get_metadata_path<S: AsRef<str>>(&self, id: S) -> PathBuf {
        self.get_upload_path(id.as_ref()).join("metadata.json")
    }
    /// Get the path to the uploaded file
    fn get_upload_file_path<S: AsRef<str>>(&self, id: S) -> PathBuf {
        self.get_upload_path(id.as_ref()).join(id.as_ref())
    }
}

/// Errors when creating an upload in a file backend
#[derive(Debug, Error, Display)]
pub enum CreateUploadError {
    /// an upload with the requested name already exists
    AlreadyExists,
    /// error creating parent directory for the upload
    CreateDirectory(#[source] io::Error),
    /// error creating metadata file
    CreateMetadataFile(#[source] io::Error),
    /// error creating file for upload contents
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
        })
    }
}

/// Errors when opening an upload stored in a file backend
#[derive(Debug, Error, Display)]
pub enum OpenUploadError {
    /// the upload was not found
    NotFound(#[source] io::Error),

    /// error while reading metadata file
    ReadMetadata(#[source] io::Error),
    /// error while opening upload file
    OpenFile(#[source] io::Error),

    /// error deserializing upload metadata
    DeserializeMetadata(#[from] serde_json::Error),
    /// error while migrating upload metadata to latest version
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
            file,
            file_path,
        })
    }
}

/// Errors when deleting an upload stored in a file backend
#[derive(Debug, Error, Display)]
pub enum DeleteUploadError {
    /// an upload at the specified id was not found
    NotFound,

    /// error deleting upload file
    DeleteFile(#[source] io::Error),
    /// error deleting metadata file
    DeleteMetadata(#[source] io::Error),
    /// error deleting upload directory
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

impl FileBackend {
    /// Returns a list of the uploads that were deleted
    ///
    /// Reasons an upload might be cleaned up:
    ///
    /// - it's expired
    /// - it's missing the file that the metadata.json points to
    ///
    /// TODO: could add locking and then delete empty/invalid metadata.json that's missing a lock
    pub async fn cleanup(&self) -> Result<Vec<CleanupError>, CleanupError> {
        while let Some(entry) = fs::read_dir(&self.path)
            .await
            .map_err(CleanupError::ReadDir)?
            .next_entry()
            .await
            .map_err(CleanupError::NextEntry)?
        {

        }

        todo!()
    }
}

/// Errors when running a cleanup task
#[derive(Debug, Error, Display)]
pub enum CleanupError {
    /// error reading directory
    ReadDir(#[source] io::Error),
    /// error reading next directory entry
    NextEntry(#[source] io::Error),
    /// failed to open metadata file
    OpenMetadata(#[source] io::Error),
    /// failed to deserialize metadata
    DeserializeMetadata(#[source] serde_json::Error),
}
pub struct CleanupUpload {
    pub id: String,
    pub reason: CleanupReason,
}
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CleanupReason {
    /// the upload has expired
    Expired,
}
pub async fn cleanup_upload<P: AsRef<Path>>(path: P) -> Result<CleanupUpload, CleanupError> {
    let metadata_path = path.as_ref().join("metadata.json");
    let metadata_file = fs::File::open(&metadata_path)
        .await
        .map_err(CleanupError::OpenMetadata)?;
    let metadata_str = String::new();
    metadata_file.read_to_end(&mut metadata_str).await?;
    let metadata: UploadMetadata = serde_json::from_str(&metadata_str)
        .map_err(CleanupError::DeserializeMetadata)?;

    todo!()
}
