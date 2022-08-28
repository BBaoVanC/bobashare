//! File backend

pub mod storage;

use std::{fs, io, path::PathBuf};

use chrono::prelude::*;
use serde::Serialize;

use self::storage::v1::{UploadFileV1, UploadV1, UploadContentsV1};

pub struct FileBackend {
    pub path: PathBuf,
}
impl FileBackend {
    /// Make a file backend, creating the directory if it doesn't exist.
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir(&path)?;
        Ok(Self { path })
    }

    /// Make a file backend, without checking if its directory exists.
    pub fn new_unchecked(path: PathBuf) -> Self {
        Self { path }
    }
}

#[derive(Debug, Clone, Serialize)]
/// An upload can contain one or many files.
pub struct Upload {
    /// The path to the upload's directory (should be part of the URL too)
    pub path: PathBuf,
    /// Sum of the size of all files in bytes, calculated at the time of upload
    /// 
    /// The accuracy of this value should not be relied on. It should only be used to display to the user.
    pub total_size: u64,
    /// When the file(s) were uploaded. This should only be used for display
    /// purposes, and should not be compared to the expiry date.
    pub creation_date: DateTime<Utc>,
    /// When the upload expires
    pub expiry_date: DateTime<Utc>,
    /// The file(s) contained in the upload
    pub files: UploadContents,
    // TODO: delete (edit?) key (jwt?)
    // TODO: should we have checksum?
}
impl Upload {
    /// Returns `true` if the file has expired.
    pub fn is_expired(&self) -> bool {
        self.expiry_date < Utc::now()
    }
}

#[derive(Debug, Clone, Serialize)]
/// Represents the contents of an upload: one or many files.
pub enum UploadContents {
    /// The upload only contains one single file.
    Single(UploadFile),
    /// The upload contains many files.
    Multiple(Vec<UploadFile>),
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadFile {
    /// The path to the file
    pub path: PathBuf,
    /// The original name of the file
    pub filename: String,
    /// Size of the file in bytes
    pub size: u64,
    // TODO: should this contain a reference to the file contents? (serde would skip it)
    // TODO: maybe add reference to the filesystem metadata, so creation/modification date could be
    // displayed, maybe even exif
}

impl From<Upload> for UploadV1 {
    fn from(upload: Upload) -> Self {
        Self {
            path: upload.path,
            total_size: upload.total_size,
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files: upload.files.into(),
        }
    }
}
impl From<UploadContents> for UploadContentsV1 {
    fn from(contents: UploadContents) -> Self {
        match contents {
            UploadContents::Single(file) => Self::Single(file.into()),
            UploadContents::Multiple(files) => Self::Multiple(
                files
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UploadFileV1>>(),
            ),
        }
    }
}
impl From<UploadFile> for UploadFileV1 {
    fn from(upload_file: UploadFile) -> Self {
        Self {
            path: upload_file.path,
            filename: upload_file.filename,
            size: upload_file.size,
        }
    }
}
