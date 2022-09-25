use std::{path::PathBuf, collections::HashMap, ffi::OsString};

use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upload {
    pub path: PathBuf,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: HashMap<OsString, UploadFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadFile {
    pub path: PathBuf,
    pub filename: String,
    pub mimetype: String,
}
