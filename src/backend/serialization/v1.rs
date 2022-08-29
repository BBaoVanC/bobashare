//! Version 1 of the upload metadata format

use std::path::PathBuf;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::{UploadFile, Upload};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadV1 {
    pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub files: Vec<UploadFileV1>,
}
impl From<UploadV1> for Upload {
    fn from(upload: UploadV1) -> Self {
        Self {
            path: upload.
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadFileV1 {
    pub path: PathBuf,
    pub filename: String,
    pub size: u64,
}
impl From<UploadFileV1> for UploadFile {
    fn from(file: UploadFileV1) -> Self {
        Self {
            path: file.path,
            filename: file.filename,
            size: file.size,
        }
    }
}
