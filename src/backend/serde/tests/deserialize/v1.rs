use pretty_assertions::assert_eq;

use crate::backend::{
    serde::{tests::util, UploadMetadata},
    Upload,
};

#[test]
fn deserialize_single_file() {
    let json = r#"
        {
            "version": "1",
            "total_size": 1234,
            "creation_date": "2022-08-29T01:02:19.824375631Z",
            "expiry_date": "2022-09-02T01:02:19.824375631Z",
            "files": [
                {
                    "path": "code.py",
                    "filename": "code.py",
                    "size": 1234
                }
            ]
        }
    "#;

    assert_eq!(
        util::single_file_example(),
        Upload::new_migrated(
            util::example_path(),
            serde_json::from_str::<UploadMetadata>(json).unwrap()
        )
        .unwrap()
    );
}

#[test]
fn deserialize_multiple_files() {
    let json = r#"
        {
            "version": "1",
            "total_size": 59909,
            "creation_date": "2022-08-29T01:02:19.824375631Z",
            "expiry_date": "2022-09-02T01:02:19.824375631Z",
            "files": [
                {
                    "path": "code.py",
                    "filename": "code.py",
                    "size": 1234
                },
                {
                    "path": "awesome.exe",
                    "filename": "awesome.exe",
                    "size": 56843
                },
                {
                    "path": "document.txt",
                    "filename": "document.txt",
                    "size": 1832
                }
            ]
        }
    "#;

    assert_eq!(
        util::multiple_files_example(),
        Upload::new_migrated(
            util::example_path(),
            serde_json::from_str::<UploadMetadata>(json).unwrap()
        )
        .unwrap()
    );
}
