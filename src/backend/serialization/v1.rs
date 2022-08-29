//! Version 1

use std::path::PathBuf;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

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
