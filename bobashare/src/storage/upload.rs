use std::collections::HashMap;

use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upload {
    pub url: String,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: HashMap<String, UploadFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadFile {
    pub filename: String,
    pub mimetype: String,
}
