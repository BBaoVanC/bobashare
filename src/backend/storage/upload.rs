use std::path::PathBuf;

use axum::{body::Bytes, extract::multipart::MultipartError};
use chrono::prelude::*;
use futures_core::Stream;
use futures_util::StreamExt;
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Upload {
    pub path: PathBuf,
    pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFile>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UploadFile {
    pub path: PathBuf,
    pub filename: String,
    pub mimetype: String,
    // only a hint
    pub size: u64,
}

#[derive(Debug, Error)]
pub enum AddFileError {
    #[error("error while decoding Multipart data")]
    MultipartError(#[from] MultipartError),
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}
impl Upload {
    pub async fn add_file_from_stream(
        &mut self,
        name: &str,
        // file: Pin<&mut impl Stream<Item = Result<Bytes, MultipartError>>>,
        mut stream: impl Stream<Item = Result<Bytes, MultipartError>> + Unpin,
    ) -> Result<(), AddFileError> {
        let mut file = File::create(self.path.join(name)).await?;
        while let Some(chunk) = stream.next().await {
            file.write_all(&chunk?).await?;
        }

        todo!()
    }
}
