use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use pretty_assertions::assert_eq;

use crate::backend::{serialization::UploadMetadata, Upload, UploadFile};

#[test]
fn serialize_latest_single_file() {
    let creation = "2022-08-29T01:02:19.824375631Z"
        .parse::<DateTime<Utc>>()
        .unwrap();

    let metadata = UploadMetadata::new(Upload {
        path: PathBuf::from("abc123xyz"),
        total_size: 1234,
        creation_date: creation,
        expiry_date: creation + Duration::days(7),
        files: vec![UploadFile {
            path: PathBuf::from("code.py"),
            filename: String::from("code.py"),
            size: 1234,
        }],
    });

    let expected = r#"{"version":"1","total_size":1234,"creation_date":"2022-08-29T01:02:19.824375631Z","expiry_date":"2022-09-05T01:02:19.824375631Z","files":[{"path":"code.py","filename":"code.py","size":1234}]}"#;

    assert_eq!(expected, serde_json::to_string(&metadata).unwrap());
}
