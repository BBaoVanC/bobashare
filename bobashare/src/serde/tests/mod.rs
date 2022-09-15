//! Tests to ensure that the metadata serialization and deserialization works as
//! expected.

mod deserialize;
mod serialize;

mod constants {
    use std::path::PathBuf;

    use chrono::{prelude::*, Duration};
    use relative_path::RelativePathBuf;

    use crate::storage::upload::{Upload, UploadFile};

    pub(crate) fn upload_path() -> PathBuf {
        PathBuf::from("abc123xyz/")
    }

    pub(crate) fn creation_date() -> DateTime<Utc> {
        "2022-08-29T01:02:19.824375631Z"
            .parse::<DateTime<Utc>>()
            .unwrap()
    }
    pub(crate) fn expiry_date() -> DateTime<Utc> {
        creation_date() + Duration::days(4)
    }

    pub(crate) fn example_upload_with_expiry() -> Upload {
        Upload {
            path: upload_path(),
            creation_date: creation_date(),
            expiry_date: Some(expiry_date()),
            files: vec![
                UploadFile {
                    path: RelativePathBuf::from("0001"),
                    filename: String::from("code.py"),
                    mimetype: String::from("text/plain"),
                },
                UploadFile {
                    path: RelativePathBuf::from("0002"),
                    filename: String::from("awesome.exe"),
                    mimetype: String::from("application/vnd.microsoft.portable-executable"),
                },
                UploadFile {
                    path: RelativePathBuf::from("0003"),
                    filename: String::from("document.txt"),
                    mimetype: String::from("text/plain"),
                },
            ],
        }
    }
    pub(crate) fn example_upload_no_expiry() -> Upload {
        let mut upload = example_upload_with_expiry();
        upload.expiry_date = None;
        upload
    }

    pub(crate) const EXAMPLE_UPLOADV1_WITH_EXPIRY_SERIALIZED: &str = r#"{"version":"1","creation_date":"2022-08-29T01:02:19.824375631Z","expiry_date":"2022-09-02T01:02:19.824375631Z","files":[{"path":"0001","filename":"code.py","mimetype":"text/plain"},{"path":"0002","filename":"awesome.exe","mimetype":"application/vnd.microsoft.portable-executable"},{"path":"0003","filename":"document.txt","mimetype":"text/plain"}]}"#;
    pub(crate) const EXAMPLE_UPLOADV1_NO_EXPIRY_SERIALIZED: &str = r#"{"version":"1","creation_date":"2022-08-29T01:02:19.824375631Z","expiry_date":null,"files":[{"path":"0001","filename":"code.py","mimetype":"text/plain"},{"path":"0002","filename":"awesome.exe","mimetype":"application/vnd.microsoft.portable-executable"},{"path":"0003","filename":"document.txt","mimetype":"text/plain"}]}"#;
}
