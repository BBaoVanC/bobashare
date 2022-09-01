//! TODO: write description

pub mod serialization;
pub mod storage;

use std::path::PathBuf;

use chrono::prelude::*;

#[derive(Debug, Clone)]
// TODO: remove non_exhaustive
#[non_exhaustive]
/// An upload can contain one or many files.
pub struct Upload {
    /// The path to the upload's directory (should be part of the URL too)
    pub path: PathBuf,
    /// Sum of the size of all files in bytes, calculated at the time of upload
    ///
    /// The accuracy of this value should not be relied on. It should only be
    /// used to display to the user.
    pub total_size: u64,
    /// When the file(s) were uploaded. This should only be used for display
    /// purposes, and should not be compared to the expiry date.
    pub creation_date: DateTime<Utc>,
    /// When the upload expires
    pub expiry_date: DateTime<Utc>,
    /// The file(s) contained in the upload
    pub files: Vec<UploadFile>,
    // TODO: delete (edit?) key (jwt?)
    // TODO: should we have checksum?
}
impl Upload {
    /// Returns `true` if the file has expired.
    pub fn is_expired(&self) -> bool {
        self.expiry_date < Utc::now()
    }
}

#[derive(Debug, Clone)]
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
