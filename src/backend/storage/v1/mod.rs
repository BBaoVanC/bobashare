use std::path::PathBuf;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::{Upload, UploadFile};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadV1 {
    pub path: PathBuf,
    pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub files: Vec<UploadFileV1>,
}
impl Into<Upload> for UploadV1 {
    fn into(self) -> Upload {
        Upload {
            path: self.path,
            total_size: self.total_size,
            creation_date: self.creation_date,
            expiry_date: self.expiry_date,
            files: self.files.into_iter().map(|f| f.into()).collect::<Vec<UploadFile>>().into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadFileV1 {
    pub path: PathBuf,
    pub filename: String,
    pub size: u64,
}
impl Into<UploadFile> for UploadFileV1 {
    fn into(self) -> UploadFile {
        UploadFile { path: self.path, filename: self.filename, size: self.size }
    }
}
