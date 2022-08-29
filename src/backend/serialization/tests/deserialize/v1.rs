use crate::backend::serialization::UploadMetadata;

#[test]
fn deserialize_single_file() {
    let json = r#"
      {
        "version": "1",
        "total_size": 1234,
        "creation_date": "2022-09-02T00:41:03.652457781Z",
        "expiry_date": "2022-09-01T00:41:03.652457781Z",
        "files": [
          {
            "path": "code.py",
            "filename": "code.py",
            "size": 1234
          }
        ]
      }
    "#;

    println!("{:#?}", serde_json::from_str::<UploadMetadata>(json).unwrap());
}
