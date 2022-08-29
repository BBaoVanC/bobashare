//! Version 1

use std::path::PathBuf;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::{v0::{UploadFileV0, UploadV0}, v0_1::{UploadFileV0_1, UploadV0_1}};

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

// impl From<UploadV0> for UploadV1 {
//     fn from(upload: UploadV0) -> Self {
//         Self {
//             total_size: upload.size_total,
//             creation_date: upload.date_created,
//             expiry_date: upload.date_expires,
//             files: upload.files.into_iter().map(Into::into).collect::<Vec<UploadFileV1>>(),
//         }
//     }
// }
// impl From<UploadFileV0> for UploadFileV1 {
//     fn from(file: UploadFileV0) -> Self {
//         Self {
//             path: PathBuf::from(file.path),
//             filename: file.filename,
//             size: file.size_bytes,
//         }
//     }
// }

impl From<UploadV0_1> for UploadV1 {
    fn from(upload: UploadV0_1) -> Self {
        Self {
            total_size: upload.size_total,
            creation_date: upload.date_created,
            expiry_date: upload.date_expires,
            files: upload.files.into_iter().map(Into::into).collect::<Vec<UploadFileV1>>(),
        }
    }
}
impl From<UploadFileV0_1> for UploadFileV1 {
    fn from(file: UploadFileV0_1) -> Self {
        Self {
            path: file.path,
            filename: file.filename,
            size: file.size_bytes,
        }
    }
}
