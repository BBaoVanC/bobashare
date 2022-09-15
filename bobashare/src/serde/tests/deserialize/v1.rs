use crate::serde::tests::constants;

#[test]
fn deserialize_upload_with_expiry() {
    let output = serde_json::from_str(&constants::EXAMPLE_UPLOADV1_WITH_EXPIRY_SERIALIZED).unwrap();
}
