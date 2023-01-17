//! Version 1

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A serializable form of [`Upload`], version 1
///
/// [`Upload`]: crate::storage::upload::Upload
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UploadV0 {
    /// name of the uploaded file
    pub filename: String,
    /// MIME type of the uploaded file
    pub mimetype: String,
    /// date the upload was created
    pub creation_date: DateTime<Utc>,
    /// date the upload expires, or [`None`] if never
    pub expiry_date: Option<DateTime<Utc>>,
    /// secret key needed to delete the upload before its expiry
    pub delete_key: String,
}
