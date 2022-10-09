use chrono::prelude::*;
use mime::Mime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upload {
    pub id: String,
    /// should be a hint
    pub size: Option<u64>,
    pub filename: String,
    pub mimetype: Mime,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
}
