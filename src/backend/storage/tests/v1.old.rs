use std::path::PathBuf;

use chrono::{prelude::*, Duration};
use indoc::indoc;

use crate::backend::storage::v1::*;


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

      println!("{:#?}", serde_json::from_str::<UploadV1>(json).unwrap());
}
