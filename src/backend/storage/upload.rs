use std::path::{Path, PathBuf};

use chrono::{prelude::*, Duration};
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::{self},
};

#[derive(Debug)]
pub struct Upload {
    pub path: PathBuf,
    // pub total_size: u64,
    pub creation_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub files: Vec<UploadFile>,
}

#[derive(Debug)]
pub struct UploadFile {
    pub path: PathBuf,
    pub file: File,
    pub filename: String,
    pub mimetype: String,
    // // only a hint
    // pub size: u64,
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
    ) -> Result<&mut UploadFile, CreateFileError> {
        let file = File::create(self.path.join(path.as_ref())).await?;

        let upload_file = UploadFile {
            path: path.as_ref().to_path_buf(),
            filename: filename.as_ref().to_string(),
            file,
            mimetype: mimetype.as_ref().to_string(),
        };
        self.files.push(upload_file);
        Ok(self.files.last_mut().unwrap())
    }
}
