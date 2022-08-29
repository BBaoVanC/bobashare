use std::path::PathBuf;

use super::{
    v0_1::{UploadFileV0_1, UploadV0_1},
    v1::{UploadFileV1, UploadV1},
    UploadMetadata,
};
use crate::backend::{Upload, UploadContents, UploadFile};

/// Convert the deserialized metadata into the actual type used by the rest of
/// the code.
impl Upload {
    // TODO: proper error handling of zero-length files array
    pub fn from_migrated(path: PathBuf, upload: UploadMetadata) -> Self {
        let mut current = upload;
        loop {
            match current {
                UploadMetadata::V0(data) => {
                    current = UploadMetadata::V0_1(UploadV0_1 {
                        size_total: data.size_total,
                        date_created: data.date_created,
                        date_expires: data.date_expires,
                        coolness: data.coolness,
                        files: data
                            .files
                            .into_iter()
                            .map(|f| UploadFileV0_1 {
                                path: PathBuf::from(f.path),
                                filename: f.filename,
                                size_bytes: f.size_bytes,
                            })
                            .collect::<Vec<UploadFileV0_1>>(),
                    })
                }
                UploadMetadata::V0_1(data) => {
                    current = UploadMetadata::V1(UploadV1 {
                        total_size: data.size_total,
                        creation_date: data.date_created,
                        expiry_date: data.date_expires,
                        files: data
                            .files
                            .into_iter()
                            .map(|f| UploadFileV1 {
                                path: f.path,
                                filename: f.filename,
                                size: f.size_bytes,
                            })
                            .collect::<Vec<UploadFileV1>>(),
                    })
                }
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
