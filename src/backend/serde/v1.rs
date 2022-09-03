//! Version 1

use std::path::PathBuf;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::UploadFile;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UploadV1 {
    pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub files: Vec<UploadFileV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UploadFileV1 {
    pub path: PathBuf,
    pub filename: String,
    pub mimetype: String,
    pub size: u64,
}

impl From<UploadFile> for UploadFileV1 {
    fn from(file: UploadFile) -> Self {
        Self {
            path: PathBuf::from(file.name),
            filename: file.filename,
            mimetype: file.mimetype,
            size: file.size,
        }
    }
}
