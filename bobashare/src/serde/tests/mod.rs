//! Tests to ensure that the metadata serialization and deserialization works as
//! expected.

// mod deserialize;
// mod serialize;

use super::UploadMetadata;

mod constants;

#[test]
fn serialize_and_deserialize_with_expiry() {
    let upload = constants::example_upload_with_expiry();
    let metadata = UploadMetadata::from_upload(upload);

    let output = serde_json::to_string(&metadata).unwrap();
}
