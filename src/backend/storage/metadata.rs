use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct UploadMetadata {
    pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub files: Vec<UploadFileMetadata>,
    // TODO: delete (edit?) key (jwt?)
    // TODO: should we have checksum?
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct UploadFileMetadata {
    pub path: String,
    pub filename: String,
    pub mimetype: String,
    // only a hint
    // pub size: u64,
    // TODO: should this contain a reference to the file contents? (serde would skip it)
    // TODO: maybe add reference to the filesystem metadata, so creation/modification date could be
    // displayed, maybe even exif
}
