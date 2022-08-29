use std::path::PathBuf;

use crate::backend::{Upload, UploadFile, UploadContents};

use super::{UploadMetadata, v1::UploadV1};

/// Convert the deserialized metadata into the actual type used by the rest of the code.
impl Upload {
    // TODO: proper error handling of zero-length files array
    pub fn from_migrated(path: PathBuf, upload: UploadMetadata) -> Self {
        let mut current = upload;
        let metadata: Upload = loop {
            match current {
                UploadMetadata::V0(data) => current = UploadMetadata::V0_1(data.into()),
                UploadMetadata::V0_1(data) => current = UploadMetadata::V1(data.into()),
                UploadMetadata::V1(data) => break data.into(),
            }
        }

        let files = metadata.files.into_iter().map(|f| UploadFile { path: f.path, filename: f.filename, size: f.size }).collect::<Vec<UploadFile>>();

        Self {
            path,
            total_size: metadata.total_size,
            creation_date: metadata.creation_date,
            expiry_date: metadata.expiry_date,
            files: match files.len() {
                0 => todo!(),
                1 => UploadContents::Single(files[0]),
                2.. => UploadContents::Multiple(files),
            }
        }
    }
}
