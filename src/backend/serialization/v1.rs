//! Version 1 of the upload metadata format

use std::path::PathBuf;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::backend::{UploadContents, Upload, UploadFile};

// use crate::backend::{UploadFile, Upload};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadV1 {
    pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub files: Vec<UploadFileV1>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadFileV1 {
    pub path: PathBuf,
    pub filename: String,
    pub size: u64,
}
// impl From<UploadFileV1> for UploadFile {
//     fn from(file: UploadFileV1) -> Self {
//         Self {
//             path: file.path,
//             filename: file.filename,
//             size: file.size,
//         }
//     }
// }

impl From<Upload> for UploadV1 {
    fn from(upload: Upload) -> Self {
        Self {
            total_size: upload.total_size,
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files: match upload.files {
                UploadContents::Single(file) => vec![file.into()],
                UploadContents::Multiple(files) => files.into_iter().map(Into::into).collect::<Vec<UploadFileV1>>(),
            },
        }
    }
}
impl From<UploadFile> for UploadFileV1 {
    fn from(file: UploadFile) -> Self {
        Self {
            path: file.path,
            filename: file.filename,
            size: file.size,
        }
    }
}
