use pretty_assertions::assert_eq;

use crate::{
    serde::{tests::constants, UploadMetadata},
    storage::upload::Upload,
};

#[test]
fn deserialize_upload_with_expiry() {
    let output: Upload = UploadMetadata::into_migrated_upload(
        constants::upload_path(),
        serde_json::from_str(constants::EXAMPLE_UPLOADV1_WITH_EXPIRY_SERIALIZED).unwrap(),
    );

    assert_eq!(output, constants::example_upload_with_expiry());
}

#[test]
fn deserialize_upload_no_expiry() {
    let output: Upload = UploadMetadata::into_migrated_upload(
        constants::upload_path(),
        serde_json::from_str(constants::EXAMPLE_UPLOADV1_NO_EXPIRY_SERIALIZED).unwrap(),
    );

    assert_eq!(output, constants::example_upload_no_expiry());
}
