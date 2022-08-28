//! Functions to convert serialized [`UploadMetadata`] versions to [`Upload`]

use super::{
    storage::{
        v1::{UploadContentsV1, UploadFileV1, UploadV1},
        UploadMetadata,
    },
    Upload, UploadContents, UploadFile,
};

impl From<UploadMetadata> for Upload {
    fn from(metadata: UploadMetadata) -> Self {
        match metadata {
            UploadMetadata::V1(data) => data.into(),
        }
    }
}

impl From<UploadV1> for Upload {
    fn from(upload: UploadV1) -> Self {
        Self {
            path: upload.path,
            total_size: upload.total_size,
            creation_date: upload.creation_date,
            expiry_date: upload.expiry_date,
            files: upload.files.into(),
        }
    }
}
impl From<UploadContentsV1> for UploadContents {
    fn from(contents: UploadContentsV1) -> Self {
        match contents {
            UploadContentsV1::Single(file) => Self::Single(file.into()),
            UploadContentsV1::Multiple(files) => Self::Multiple(
                files
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<UploadFile>>(),
            ),
        }
    }
}
impl From<UploadFileV1> for UploadFile {
    fn from(file: UploadFileV1) -> Self {
        Self {
            path: file.path,
            filename: file.filename,
            size: file.size,
        }
    }
}
