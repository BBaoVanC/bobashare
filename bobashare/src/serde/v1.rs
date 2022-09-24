//! Version 1

use std::ffi::OsString;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::storage::upload::UploadFile;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UploadV1 {
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFileV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UploadFileV1 {
    pub path: OsString,
    pub filename: String,
    pub mimetype: String,
}

// impl UploadFileV1 {
//     pub fn from_file(file: UploadFile) -> Self {
//         Self {
//             path: file.path,
//             filename: file.filename,
//             mimetype: file.mimetype,
//         }
//     }
// }
// impl From<UploadFileV1> for UploadFile {
//     fn from(upload: UploadFileV1) -> Self {
//         Self {
//             path: upload.path,
//             filename: upload.filename,
//             mimetype: upload.mimetype,
//         }
//     }
// }
