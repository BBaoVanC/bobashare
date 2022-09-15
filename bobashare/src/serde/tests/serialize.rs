use tokio_test::block_on;

use super::constants;
use crate::serde::UploadMetadata;

#[test]
fn serialize_into_latest_with_expiry() {
    let upload = constants::example_upload_with_expiry();
    let metadata = block_on(async { UploadMetadata::from_upload(upload).await.unwrap() });

    let output = serde_json::to_string_pretty(&metadata).unwrap();

    println!("{}", output);
}
