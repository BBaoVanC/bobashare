use chrono::{DateTime, Utc};
use mime::Mime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upload {
    pub id: String,
    pub filename: String,
    pub mimetype: Mime,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
}
