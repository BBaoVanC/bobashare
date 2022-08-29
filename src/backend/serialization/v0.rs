//! Version for testing migration into V1

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadV0 {
    pub size_total: u64,
    pub date_created: DateTime<Utc>,
    pub date_expires: DateTime<Utc>,
    pub files: Vec<UploadFileV0>,
    pub coolness: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadFileV0 {
    pub path: String,
    pub filename: String,
    pub size_bytes: u64,
}
