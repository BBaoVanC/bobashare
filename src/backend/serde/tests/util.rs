use std::path::PathBuf;

use chrono::{prelude::*, Duration};

use crate::backend::{Upload, UploadFile};

pub const EXAMPLE_NAME: &str = "abc123xyz";
pub fn example_path() -> PathBuf {
    PathBuf::from(EXAMPLE_NAME)
}

pub fn example_creation_date() -> DateTime<Utc> {
    "2022-08-29T01:02:19.824375631Z"
        .parse::<DateTime<Utc>>()
        .unwrap()
}
pub fn example_expiry_date() -> DateTime<Utc> {
    example_creation_date() + Duration::days(4)
}

pub fn single_file_example() -> Upload {
    Upload {
        url: String::from(EXAMPLE_NAME),
        total_size: 1234,
        creation_date: example_creation_date(),
        expiry_date: example_expiry_date(),
        files: vec![UploadFile {
            path: PathBuf::from("code.py"),
            filename: String::from("code.py"),
            mimetype: "text/plain",
            size: 1234,
        }],
    }
}

pub fn multiple_files_example() -> Upload {
    Upload {
        url: String::from(EXAMPLE_NAME),
        total_size: 59909,
        creation_date: example_creation_date(),
        expiry_date: example_expiry_date(),
        files: vec![
            UploadFile {
                path: PathBuf::from("code.py"),
                filename: String::from("code.py"),
                mimetype: "text/plain",
                size: 1234,
            },
            UploadFile {
                path: PathBuf::from("awesome.exe"),
                filename: String::from("awesome.exe"),
                mimetype: "text/plain",
                size: 56843,
            },
            UploadFile {
                path: PathBuf::from("document.txt"),
                filename: String::from("document.txt"),
                mimetype: "text/plain",
                size: 1832,
            },
        ],
    }
}
