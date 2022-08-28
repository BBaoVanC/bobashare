//! Test serialization of [`File`]

use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use indoc::indoc;

use crate::backend::{Upload, UploadContents, UploadFile};

#[test]
fn test_single_file_serialization() {
    let now = Utc::now();

    let upload = Upload {
        path: PathBuf::from("abc123xyz"), // would be a directory
        total_size: 1234,
        creation_date: now - Duration::days(4),
        expiry_date: now + Duration::days(3),
        files: UploadContents::Single(UploadFile {
            path: PathBuf::from("awesome_code.rs"),
            filename: String::from("awesome_code.rs"),
            size: 1234,
        }),
    };

    println!("{}", serde_json::to_string_pretty(&upload).unwrap());
}

#[test]
fn test_multiple_file_serialization() {
    let now = Utc::now();

    let upload = Upload {
        path: PathBuf::from("abc123xyz"),
        total_size: 512,
        creation_date: now - Duration::days(4),
        expiry_date: now + Duration::days(3),
        files: UploadContents::Multiple(vec![
            UploadFile {
                path: PathBuf::from("frontend.js"),
                filename: String::from("frontend.js"),
                size: 256,
            },
            UploadFile {
                path: PathBuf::from("backend.py"),
                filename: String::from("backend.py"),
                size: 128,
            },
        ]),
    };

    println!("{}", serde_json::to_string_pretty(&upload).unwrap());
}

#[test]
fn test_single_file_deserialization() {
    let json = indoc! {r#"{
        "path": "abc123xyz",
        "total_size": 1234,
        "creation_date": "2022-08-24T01:23:34.550173644Z",
        "expiry_date": "2022-08-31T01:23:34.550173644Z",
        "files": {
          "Single": {
            "path": "awesome_code.rs",
            "filename": "awesome_code.rs",
            "size": 1234
          }
        }
      }"#};

      println!("{:#?}", serde_json::from_str::<Upload>(json).unwrap());
}
