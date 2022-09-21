use std::path::PathBuf;

use chrono::prelude::*;
use relative_path::RelativePathBuf;

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

impl Upload {}
