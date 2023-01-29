use super::constants;
use crate::serde::{MigrateError, UploadMetadata};

mod v0;

#[test]
fn deserialize_unknown_version_is_unknown() {
    match serde_json::from_str::<UploadMetadata>(constants::EXAMPLE_UNKNOWN_VERSION).unwrap() {
        UploadMetadata::Unknown => {}
        m => panic!("should be unknown, but was deserialized into: {:?}", m),
    }
}

#[test]
fn migrate_unknown_version_should_fail() {
    match UploadMetadata::Unknown.into_migrated_upload("foobar".to_string()) {
        Err(MigrateError::UnknownVersion) => {}
        e => panic!("should be MigrateError::UnknownVersion, but was {:?}", e),
    }
}
