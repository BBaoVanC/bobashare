use std::path::{PathBuf, Path};

use axum::{body::Bytes, extract::multipart::MultipartError};
use chrono::prelude::*;
use futures_core::Stream;
use futures_util::StreamExt;
use relative_path::RelativePathBuf;
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::{self, AsyncWriteExt},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Upload {
    pub path: RelativePathBuf,
    pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFile>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UploadFile {
    pub path: RelativePathBuf,
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
    pub async fn create_file<P: AsRef<Path>, S: AsRef<str>>(&mut self, name: P, mimetype: S) -> Result<(File, UploadFile), io::Error> {
        let file = File::create(self.path.join(name)).await?;
        let upload_file = UploadFile { path: PathBuf::from(name.as_ref()), filename: String::from(name.as_ref().to_str()), mimetype, size: 0 }
        Ok((file, upload_file))
    }

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
