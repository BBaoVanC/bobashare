use bobashare::backend::{
    generate_randomized_name,
    storage::file::{CreateUploadError, FileBackend},
};
use chrono::Duration;

use crate::CreateUpload;

async fn create_upload(backend: FileBackend, args: CreateUpload) {
    loop {
        if let Err(CreateUploadError::AlreadyExists) = backend
            .create_upload(&args.name, args.expiry.map(|e| Duration::days(e.into())))
            .await
        {
            continue;
        } else {
            return Err()
        }
    }
}
