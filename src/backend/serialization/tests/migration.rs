use std::path::PathBuf;

use chrono::{prelude::*, Duration};

use crate::backend::{
    serialization::{
        v0::{UploadFileV0, UploadV0},
        UploadMetadata,
    },
    Upload, UploadContents, UploadFile,
};

#[test]
fn migrate_v0_to_v1() {
    let creation = Utc::now();

    let zero = UploadMetadata::V0(UploadV0 {
        size_total: 1234,
        date_created: creation,
        date_expires: creation + Duration::days(7),
        files: vec![UploadFileV0 {
            path: String::from("code.py"),
            filename: String::from("code.py"),
            size_bytes: 1234,
        }],
        coolness: 0,
    });

    let one: Upload = Upload::from_migrated(PathBuf::from("abc123xyz"), zero);
    println!("{:#?}", one);
}
