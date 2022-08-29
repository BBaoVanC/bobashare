use std::path::PathBuf;

use chrono::{prelude::*, Duration};

use crate::backend::{serialization::UploadMetadata, Upload, UploadContents, UploadFile};

use pretty_assertions::assert_eq;

#[test]
fn serialize_latest_single_file() {
    let creation = "2022-08-29T01:02:19.824375631Z".parse::<DateTime<Utc>>().unwrap();

    let metadata = UploadMetadata::new_latest(Upload {
        path: PathBuf::from("abc123xyz"),
        total_size: 1234,
        creation_date: creation,
        expiry_date: creation + Duration::days(7),
        files: UploadContents::Single(UploadFile {
            path: PathBuf::from("code.py"),
            filename: String::from("code.py"),
            size: 1234,
        }),
    });

    let expected = r#"{"version":"1","total_size":1234,"creation_date":"2022-08-29T01:02:19.824375631Z","expiry_date":"2022-09-05T01:02:19.824375631Z","files":[{"path":"code.py","filename":"code.py","size":1234}]}"#;

    assert_eq!(expected, serde_json::to_string(&metadata).unwrap());
}
