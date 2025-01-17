//! Type that stores information (metadata) about an upload, and related methods

use chrono::{DateTime, Utc};
use mime::Mime;

/// Metadata about an upload
// TODO: maybe store uploader ip for spam reasons
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Upload {
    /// ID of the upload
    pub id: String,
    /// name of the uploaded file
    pub filename: String,
    /// MIME type of the uploaded file
    pub mimetype: Mime,
    /// date the upload was created
    pub creation_date: DateTime<Utc>,
    /// date the upload expires, or [`None`] if never
    pub expiry_date: Option<DateTime<Utc>>,
    /// secret key needed to delete the upload before it expires
    pub delete_key: String,
}
impl Upload {
    /// Check whether or not the upload is expired.
    ///
    /// Returns [`true`] if the upload has expired and should be deleted.
    pub fn is_expired(&self) -> bool {
        // if None (never expires) then not expired (false)
        // otherwise if expiry is before now, then it is expired
        self.expiry_date.is_some_and(|e| e < Utc::now())
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeDelta;

    use super::*;

    // in the past
    fn creation_date() -> DateTime<Utc> {
        "2022-08-22T01:02:19.824375631Z"
            .parse::<DateTime<Utc>>()
            .unwrap()
    }
    fn test_upload_no_expiry() -> Upload {
        Upload {
            id: String::from("abc123xyz"),
            filename: String::from("document.txt"),
            mimetype: "text/plain".parse().unwrap(),
            creation_date: creation_date(),
            expiry_date: None,
            delete_key: String::from("*^G^(MNCW#$(GMm9g87ctm4g98c43g789"),
        }
    }
    fn test_upload_expired() -> Upload {
        let mut upload = test_upload_no_expiry();
        upload.expiry_date = Some(
            "2022-08-29T01:02:19.824375631Z"
                .parse::<DateTime<Utc>>()
                .unwrap(),
        );
        upload
    }
    fn test_upload_not_expired() -> Upload {
        let mut upload = test_upload_expired();
        upload.expiry_date =
            Some(upload.expiry_date.unwrap() + TimeDelta::try_weeks(52000).unwrap());
        upload
    }

    #[test]
    fn no_expiry_is_not_expired() {
        assert!(!test_upload_no_expiry().is_expired());
    }
    #[test]
    fn not_expired() {
        assert!(!test_upload_not_expired().is_expired());
    }
    #[test]
    fn expired() {
        assert!(test_upload_expired().is_expired());
    }
}
