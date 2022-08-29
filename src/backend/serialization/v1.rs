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
