use pretty_assertions::assert_eq;

use crate::serde::{tests::constants, UploadMetadata};

#[test]
fn deserialize_upload_with_expiry() {
    let metadata: UploadMetadata =
        serde_json::from_str(constants::EXAMPLE_UPLOADV0_WITH_EXPIRY_SERIALIZED).unwrap();
    let output = metadata
        .into_migrated_upload(constants::upload_id())
        .unwrap();

    assert_eq!(output.0, constants::example_upload_with_expiry());
}

#[test]
fn deserialize_upload_no_expiry() {
    let metadata: UploadMetadata =
        serde_json::from_str(constants::EXAMPLE_UPLOADV0_NO_EXPIRY_SERIALIZED).unwrap();
    let output = metadata
        .into_migrated_upload(constants::upload_id())
        .unwrap();

    assert_eq!(output.0, constants::example_upload_no_expiry());
}
