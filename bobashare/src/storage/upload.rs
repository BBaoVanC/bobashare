use std::path::{Path, PathBuf};

use chrono::prelude::*;
use thiserror::Error;
use tokio::{
    fs::File,
    io::{self},
};
use tracing::{event, instrument, Level};

#[derive(Debug, Clone)]
pub struct Upload {
    pub path: PathBuf,
    // pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFile>,
}
#[derive(Debug, Clone)]
pub struct UploadFile {
    pub path: PathBuf,
    pub filename: String,
    pub mimetype: String,
}
