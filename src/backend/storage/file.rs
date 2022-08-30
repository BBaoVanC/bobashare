//! A backend that stores as files

use std::{fs, io, path::PathBuf};

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
    ///
    /// If the directory doesn't already exist, then almost any operation will
    /// panic.
    pub fn new_unchecked(path: PathBuf) -> Self {
        Self { path }
    }
}
