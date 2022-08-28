#[cfg(test)]
mod tests;

use std::{fs, io, path::PathBuf};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Upload {
    /// The original name of the file before it was uploaded. This should be a
    /// filename and extension only; it should not include a directory.
    pub filename: PathBuf,
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
impl Upload {
    /// Returns `true` if the file has expired.
    pub fn is_expired(&self) -> bool {
        self.expiry_date < Utc::now()
    }
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

    /// Make a file backend, without checking if its directory exists.
    pub fn new_unchecked(path: PathBuf) -> Self {
        Self { path }
    }
}
