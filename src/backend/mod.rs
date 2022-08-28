#[cfg(test)]
mod tests;

use std::{fs, io, path::PathBuf};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct UploadMetadata {
    /// The original name of the file before it was uploaded. This should be a filename and extension only; it should not include a directory.
    pub filename: PathBuf,
    /// The file extension, which should be used to also look the actual file up
    /// under the `files_dir` in [`FileBackend`].
    ///
    /// The dot should not be included (so instead of `.rs`, it should be just
    /// `rs`).
    pub file_extension: String,
    /// File size in bytes
    pub size: u64,
    // pub mime_type: FileFormat,
    /// The media type of the file (MIME type)
    pub mime_type: String,
    // pub charset: Option<String>, // maybe needed for plaintext
    /// When the file was uploaded. This should only be used for display
    /// purposes, and should not be compared to the expiry date.
    pub upload_date: DateTime<Utc>,
    /// When the file expires
    pub expiry_date: DateTime<Utc>,
    // TODO: delete (edit?) key
    // TODO: should we have checksum?
}
impl UploadMetadata {
    /// Returns `true` if the file has expired.
    pub fn is_expired(&self) -> bool {
        self.expiry_date < Utc::now()
    }
}

/*
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FileFormat {
    Plaintext { mime: String, charset: String },
    Mime(String),
}
*/

pub struct FileBackend {
    pub files_dir: PathBuf,
    pub metadata_dir: PathBuf,
}
impl FileBackend {
    /// Make a file backend, and create the `files` and `metadata` directories
    /// if they do not exist.
    pub fn new_checked(files_dir: PathBuf, metadata_dir: PathBuf) -> Result<Self, io::Error> {
        fs::create_dir(&files_dir)?;
        fs::create_dir(&metadata_dir)?;
        Ok(Self {
            files_dir,
            metadata_dir,
        })
    }

    // pub fn list_files(&self) -> Result<Vec<File>, io::Error> {}
}
