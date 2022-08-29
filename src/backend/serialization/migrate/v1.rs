use std::path::PathBuf;

use crate::backend::{serialization::UploadMetadata, Upload, UploadContents, UploadFile};

impl Upload {
    /// # Panics
    /// 
    /// - If the files array is empty
    pub fn from_metadata(metadata: UploadMetadata, path: PathBuf) -> Self {
        match metadata {
            UploadMetadata::V1(data) => Self {
                path: path,
                total_size: data.total_size,
                creation_date: data.creation_date,
                expiry_date: data.expiry_date,
                files: if data.files.len() > 1 {
                    UploadContents::Multiple(data.files.into_iter().map(Into::into).collect::<Vec<UploadFile>>())
                } else {
                    UploadContents::Single(data.files[0].into())
                },
            },
        }
    }
}
