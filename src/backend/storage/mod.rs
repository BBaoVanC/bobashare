//! Modules that handle storing uploaded files and serialized metadata.

use async_trait::async_trait;
use chrono::{prelude::*, Duration};
use thiserror::Error;
use tracing::instrument;

use super::{serialization::UploadMetadata, Upload, UploadFile};

pub mod file;

#[derive(Debug, Error)]
pub enum CreateUploadError {
    #[error("the list of files to upload was empty")]
    ZeroFiles,
    #[error("an upload with the requested name already exists")]
    AlreadyExists,
}
#[derive(Debug, Error)]
pub enum QueryUploadError {
    #[error("error while doing i/o")]
    IoError(#[from] std::io::Error),
}
#[derive(Debug, Error)]
pub enum DeleteUploadError {
    #[error("error while doing i/o")]
    IoError(#[from] std::io::Error),
}

#[async_trait]
pub trait StorageBackend {
    type StreamOutput;

    async fn create_upload(&self, url: String, files: Vec<UploadFile>, expiry: Duration) -> Result<Upload, CreateUploadError>;
    async fn check_exists(&self, url: String) -> Result<bool, QueryUploadError>;
    async fn query_metadata(&self, url: String) -> Result<UploadMetadata, QueryUploadError>;
    async fn stream_file(
        &self,
        url: String,
        file: String,
    ) -> Result<Self::StreamOutput, QueryUploadError>;
    async fn delete_upload(&self, url: String) -> Result<(), DeleteUploadError>;
}
