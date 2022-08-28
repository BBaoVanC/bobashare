//! Test serialization of the latest structs

use std::path::PathBuf;

use chrono::{prelude::*, Duration};

use crate::backend::{storage::UploadMetadata, Upload, UploadContents, UploadFile};

#[test]
fn deserialize_single_file() {
    let json = r#"
    {
      "version": "1",
      "path": "abc123xyz",
      "total_size": 1234,
      "creation_date": "2022-08-24T19:06:31.929487373Z",
      "expiry_date": "2022-08-31T19:06:31.929487373Z",
      "files": {
        "Single": {
          "path": "awesome_code.rs",
          "filename": "awesome_code.rs",
          "size": 1234
        }
      }
    }
    "#;

    println!(
        "{:#?}",
        serde_json::from_str::<UploadMetadata>(json).unwrap()
    );
}

#[test]
fn serialize_multiple_files() {
    let json = r#"
    {
      "version": "1",
      "path": "abc123xyz",
      "total_size": 512,
      "creation_date": "2022-08-24T19:06:31.929479614Z",
      "expiry_date": "2022-08-31T19:06:31.929479614Z",
      "files": {
        "Multiple": [
          {
            "path": "frontend.js",
            "filename": "frontend.js",
            "size": 256
          },
          {
            "path": "backend.py",
            "filename": "backend.py",
            "size": 128
          }
        ]
      }
    }
    "#;

    println!(
        // "{}",
        // serde_json::to_string_pretty(&UploadMetadata::latest(upload)).unwrap()
    );
}
