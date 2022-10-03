use chrono::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upload {
    pub url: String,
    /// should be a hint
    pub size: Option<usize>,
    pub filename: String,
    pub mimetype: String,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
}
