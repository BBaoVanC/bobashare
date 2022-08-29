use std::path::PathBuf;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::v0::{UploadV0, UploadFileV0};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadV0_1 {
    pub size_total: u64,
    pub date_created: DateTime<Utc>,
    pub date_expires: DateTime<Utc>,
    pub files: Vec<UploadFileV0_1>,
    pub coolness: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadFileV0_1 {
    pub path: PathBuf,
    pub filename: String,
    pub size_bytes: u64,
}
