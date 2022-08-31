use std::path::PathBuf;

use super::UploadMetadata;
use crate::backend::{Upload, UploadContents, UploadFile};

/// Convert the deserialized metadata into the actual type used by the rest of
/// the code.
impl Upload {
    // TODO: proper error handling of zero-length files array
    pub fn new_migrated(path: PathBuf, upload: UploadMetadata) -> Self {
        let mut current = upload;
        loop {
            match current {
                UploadMetadata::V0(data) => current = UploadMetadata::V0_1(data.into()),
                UploadMetadata::V0_1(data) => current = UploadMetadata::V1(data.into()),
                UploadMetadata::V1(data) => {
                    let files = data
                        .files
                        .into_iter()
                        .map(|f| UploadFile {
                            path: f.path,
                            filename: f.filename,
                            size: f.size,
                        })
                        .collect::<Vec<UploadFile>>();
                    break Upload {
                        path,
                        total_size: data.total_size,
                        creation_date: data.creation_date,
                        expiry_date: data.expiry_date,
                        // TODO: handle files.len() == 0
                        files: if files.len() > 1 {
                            UploadContents::Multiple(files)
                        } else {
                            UploadContents::Single(files[0].clone()) // TODO: don't clone
                        },
                    };
                }
            }
        }
    }
}
