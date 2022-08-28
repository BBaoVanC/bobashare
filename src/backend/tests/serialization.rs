//! Test serialization of [`File`]

use std::path::PathBuf;

use chrono::{prelude::*, Duration};

use crate::backend::Upload;

#[test]
fn test_serialization() {
    let now = Utc::now();
    let metadata = Upload {
        filename: PathBuf::from("super_cool_code.rs"),
        file_extension: String::from("rs"),
        size: 64,
        mime_type: String::from("text/plain"),
        upload_date: now - Duration::days(4),
        expiry_date: now + Duration::days(3),
    };

    println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
}
