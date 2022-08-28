//! Test serialization of the latest structs

use std::path::PathBuf;

use chrono::{prelude::*, Duration};

use crate::backend::{storage::UploadMetadata, Upload, UploadContents, UploadFile};

#[test]
fn serialize_single_file() {
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

    println!(
        "{}",
        serde_json::to_string_pretty(&UploadMetadata::from(upload)).unwrap()
    );
}

#[test]
fn serialize_multiple_files() {
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

    println!(
        "{}",
        serde_json::to_string_pretty(&UploadMetadata::from(upload)).unwrap()
    );
}
