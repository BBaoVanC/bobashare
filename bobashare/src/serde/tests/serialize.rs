use pretty_assertions::assert_eq;

use super::constants;
use crate::serde::UploadMetadata;

#[test]
fn serialize_into_latest_with_expiry() {
    let upload = constants::example_upload_with_expiry();
    let metadata = UploadMetadata::from_upload(upload);

    let output = serde_json::to_string(&metadata).unwrap();

    assert_eq!(output, constants::EXAMPLE_UPLOADV1_WITH_EXPIRY_SERIALIZED);
}

#[test]
fn serialize_into_latest_no_expiry() {
    let upload = constants::example_upload_no_expiry();
    let metadata = UploadMetadata::from_upload(upload);

    let output = serde_json::to_string(&metadata).unwrap();

    assert_eq!(output, constants::EXAMPLE_UPLOADV1_NO_EXPIRY_SERIALIZED);
}
