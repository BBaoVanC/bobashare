use std::path::{Path, PathBuf};

use chrono::{prelude::*, Duration};
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::{self},
};

#[derive(Debug, Clone)]
pub struct Upload {
    pub path: PathBuf,
    // pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFile>,
}
impl Drop for Upload {
    /// Save upload metadata when the object is dropped.
    fn drop(&mut self) {
    }
}

#[derive(Debug, Clone)]
pub struct UploadFile {
    pub path: PathBuf,
    pub filename: String,
    pub mimetype: String,
}

#[derive(Debug)]
pub struct UploadFileHandle<'h> {
    pub metadata: UploadFile,
    pub file: File,
    files_vec: &'h mut Vec<UploadFile>
}
impl<'h> UploadFileHandle<'_> {
    pub fn new(metadata: UploadFile, file: File, files_vec: &'h mut Vec<UploadFile>) -> UploadFileHandle<'h> {
        UploadFileHandle {
            metadata,
            file,
            files_vec,
        }
    }
}
impl Drop for UploadFileHandle<'_> {
    /// Automatically add file to the [`Upload`] when the handle is dropped.
    fn drop(&mut self) {
        self.files_vec.push(self.metadata.clone());
    }
}

#[derive(Debug, Error)]
pub enum CreateFileError {
    #[error("error while doing i/o")]
    IoError(#[from] io::Error),
}
impl Upload {
    // pub async fn create<P: AsRef<Path>>(
    //     path: P,
    //     expiry: Option<Duration>,
    // ) -> Result<Self, io::Error> {
    //     let creation_date = Utc::now();

    //     if let Err(e) = fs::create_dir(&path).await {
    //         if e.kind() != io::ErrorKind::AlreadyExists {
    //             return Err(e);
    //         }
    //     }

    //     Ok(Self {
    //         path: path.as_ref().to_path_buf(),
    //         creation_date: Utc::now(),
    //         expiry_date: expiry.map(|e| creation_date + e),
    //         files: Vec::new(),
    //     })
    // }

    pub async fn create_file<P: AsRef<Path>, S: AsRef<str>>(
        &mut self,
        path: P,
        filename: S,
        mimetype: S,
    ) -> Result<UploadFileHandle, CreateFileError> {
        let file = File::create(self.path.join(path.as_ref())).await?;

        let metadata = UploadFile {
            path: path.as_ref().to_path_buf(),
            filename: filename.as_ref().to_string(),
            mimetype: mimetype.as_ref().to_string(),
        };
        let handle = UploadFileHandle::new(metadata, file, &mut self.files);
        Ok(handle)
    }
}
