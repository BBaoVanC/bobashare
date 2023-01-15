//! A backend where uploads are stored as files on disk

use std::path::PathBuf;

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
    io::{self, AsyncReadExt},
};
use tracing::{event, instrument, Instrument, Level};

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
    /// Get the path to the `metadata.lock` file
    fn get_lock_path<S: AsRef<str>>(&self, id: S) -> PathBuf {
        self.get_upload_path(id.as_ref()).join("metadata.lock")
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
    /// error creating lock file
    CreateLockFile(#[source] io::Error),
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

        let lock_path = self.get_lock_path(id.as_ref());
        let lock_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
            .await
            .map_err(CreateUploadError::CreateLockFile)?;
        drop(lock_file);

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
            lock_path,
        })
    }
}

/// Errors when opening an upload stored in a file backend
#[derive(Debug, Error, Display)]
pub enum OpenUploadError {
    /// the upload was not found
    NotFound(#[source] io::Error),
    /// the upload is locked
    Locked,

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
        let lock_path = self.get_lock_path(id.as_ref());
        if fs::metadata(lock_path).await.is_ok() {
            return Err(OpenUploadError::Locked);
        }

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
        // TODO: if metadata.1 (was migrated), save new migrated metadata
        Ok(metadata.0)
    }

    /// does not check if the upload is expired, do that yourself
    pub async fn open_upload<S: AsRef<str>>(
        &self,
        id: S,
        write: bool,
    ) -> Result<UploadHandle, OpenUploadError> {
        let lock_path = self.get_lock_path(id.as_ref());
        if fs::metadata(&lock_path).await.is_ok() {
            return Err(OpenUploadError::Locked);
        }

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
            metadata: metadata.0,
            metadata_file,
            file,
            file_path,
            lock_path,
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

/// Critical errors when validating an upload that mean we can't determine
/// whether it's valid or not
#[derive(Debug, Error, Display)]
pub enum ValidateError {
    /// failed to open metadata file
    OpenMetadata(#[source] io::Error),
    /// failed to migrate metadata
    MigrateMetadata(#[from] MigrateError),
}
#[derive(Debug, Display)]
pub enum ValidateResult {
    /// the upload is valid
    Valid,
    /// the upload is locked
    Locked,
    /// the upload is invalid and should be deleted
    Invalid(InvalidReason),
}
/// Reasons why an upload might be invalid and need to be deleted
#[derive(Debug, Error, Display)]
pub enum InvalidReason {
    /// the upload has expired
    Expired,
    /// the upload is locked
    Locked,

    /// the upload is missing metadata.json
    MissingMetadata(#[source] io::Error),
    /// the upload metadata isn't valid or cannot be deserialized
    InvalidMetadata(#[from] serde_json::Error),

    /// the upload is missing a file
    MissingFile,
}
impl From<InvalidReason> for Result<ValidateResult, ValidateError> {
    fn from(val: InvalidReason) -> Self {
        Ok(ValidateResult::Invalid(val))
    }
}
/// Checks if an upload is valid or if it should be cleaned up
///
/// Checks:
///
/// 1. check if there is an upload file
/// 2. check if there is a metadata file
/// 3. make sure the file isn't locked
/// 4. check if the metadata file is valid (meaning: can be deserialized)
/// 5. migrate the metadata if needed
/// 6. check if the upload has expired
///
/// TODO: what should we do if it's a file and not a directory?
impl FileBackend {
    pub async fn validate_upload<S: AsRef<str>>(
        &self,
        id: S,
    ) -> Result<ValidateResult, ValidateError> {
        // ^ we're gonna have an ugly Ok() layer for actual valid/invalid :skull:
        let id = id.as_ref();

        // 1. check if there is an upload file
        let file_path = self.get_upload_file_path(id);
        if !file_path.exists() {
            return InvalidReason::MissingFile.into();
        }

        // 2, 3, 4, 5. check if there is a metadata file and if it's valid, and that
        // there's no lock file
        let metadata = match self.read_upload_metadata(id).await {
            Ok(m) => m,
            Err(err) => match err {
                OpenUploadError::NotFound(e) => return InvalidReason::MissingMetadata(e).into(),
                OpenUploadError::Locked => return Ok(ValidateResult::Locked),
                OpenUploadError::OpenFile(_) => unreachable!(),
                OpenUploadError::ReadMetadata(e) => return Err(ValidateError::OpenMetadata(e)),
                OpenUploadError::MigrateMetadata(e) => {
                    return Err(ValidateError::MigrateMetadata(e))
                }
                OpenUploadError::DeserializeMetadata(e) => {
                    return InvalidReason::InvalidMetadata(e).into()
                }
            },
        };

        // 6. check if the upload is expired
        if metadata.is_expired() {
            return InvalidReason::Expired.into();
        }

        Ok(ValidateResult::Valid)
    }
}

/// Errors when running a repository cleanup task
#[derive(Debug, Error, Display)]
pub enum CleanupError {
    /// error reading directory
    ReadDir(#[source] io::Error),
    /// error reading next directory entry
    NextEntry(#[source] io::Error),
}
impl FileBackend {
    /// Validate all the uploads in the repository and return a list of ones to
    /// be deleted
    ///
    /// See [`FileBackend::validate_upload`] for the checks that are performed
    #[instrument(skip(self))]
    pub async fn cleanup(&self) -> Result<(), CleanupError> {
        let mut delete_queue = Vec::new();
        let mut read_dir = fs::read_dir(&self.path)
            .await
            .map_err(CleanupError::ReadDir)?;
        while let Some(entry) = read_dir
            .next_entry()
            .await
            .map_err(CleanupError::NextEntry)?
        {
            let span = tracing::span!(Level::DEBUG, "validate", name = ?entry.file_name());
            async {
                let Some(id) = entry.file_name().to_str().map(ToString::to_string) else {
                    event!(Level::WARN, "invalid file name");
                    return;
                };

                match self.validate_upload(&id).await {
                    Ok(res) => match res {
                        ValidateResult::Valid => event!(Level::DEBUG, "valid"),
                        ValidateResult::Locked => event!(Level::INFO, "locked"),
                        ValidateResult::Invalid(reason) => {
                            event!(Level::DEBUG, "will delete: {reason}");
                            delete_queue.push(id);
                        }
                    },
                    Err(err) => {
                        event!(Level::ERROR, "error validating: {err}");
                    }
                }
            }
            .instrument(span)
            .await;
        }

        for id in delete_queue {
            let span = tracing::span!(Level::INFO, "delete", id = ?id);
            // TODO: some way to return a list of failed deletes
            async {
                event!(Level::INFO, id, "deleting");
                if let Err(err) = fs::remove_dir_all(self.get_upload_path(&id)).await {
                    event!(Level::ERROR, id, "error deleting: {err}");
                }
            }
            .instrument(span)
            .await
        }

        Ok(())
    }
}
