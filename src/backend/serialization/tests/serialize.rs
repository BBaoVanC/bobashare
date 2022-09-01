use pretty_assertions::assert_eq;

use crate::backend::serialization::{tests::util, UploadMetadata};

#[test]
fn serialize_latest_single_file() {
    let metadata = UploadMetadata::new(util::single_file_example());
    let expected = r#"{"version":"1","total_size":1234,"creation_date":"2022-08-29T01:02:19.824375631Z","expiry_date":"2022-09-02T01:02:19.824375631Z","files":[{"path":"code.py","filename":"code.py","size":1234}]}"#;

    assert_eq!(expected, serde_json::to_string(&metadata).unwrap());
}

#[test]
fn serialize_latest_multiple_files() {
    let metadata = UploadMetadata::new(util::multiple_files_example());
    let expected = r#"{"version":"1","total_size":59909,"creation_date":"2022-08-29T01:02:19.824375631Z","expiry_date":"2022-09-02T01:02:19.824375631Z","files":[{"path":"code.py","filename":"code.py","size":1234},{"path":"awesome.exe","filename":"awesome.exe","size":56843},{"path":"document.txt","filename":"document.txt","size":1832}]}"#;

    assert_eq!(expected, serde_json::to_string(&metadata).unwrap());
}
