use std::path::{Path, PathBuf};

use chrono::prelude::*;
use relative_path::{FromPathError, RelativePathBuf};
use thiserror::Error;
use tokio::{fs::{File, self}, io::{self, AsyncWriteExt}};

use super::handle::UploadFileHandle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upload {
    pub path: PathBuf,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadFile {
    pub path: RelativePathBuf,
    pub filename: String,
    pub mimetype: String,
}

impl Upload {
}
