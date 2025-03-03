//! Modules that handle storing uploaded files and serialized metadata.

pub mod file;

pub mod upload {
    //! Type that stores information (metadata) about an upload, and related
    //! methods
    use chrono::{DateTime, Utc};
    use mime::Mime;

    /// Metadata about an upload
    // TODO: maybe store uploader ip for spam reasons
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Upload {
        /// ID of the upload
        pub id: String,
        /// name of the uploaded file
        pub filename: String,
        /// MIME type of the uploaded file
        pub mimetype: Mime,
        /// date the upload was created
        pub creation_date: DateTime<Utc>,
        /// date the upload expires, or [`None`] if never
        pub expiry_date: Option<DateTime<Utc>>,
        /// secret key needed to delete the upload before it expires
        pub delete_key: String,
    }
    impl Upload {
        /// Check whether or not the upload is expired.
        ///
        /// Returns [`true`] if the upload has expired and should be deleted.
        pub fn is_expired(&self) -> bool {
            // if None (never expires) then not expired (false)
            // otherwise if expiry is before now, then it is expired
            self.expiry_date.is_some_and(|e| e < Utc::now())
        }
    }

    #[cfg(test)]
    mod tests {
        use chrono::TimeDelta;

        use super::*;

        // in the past
        fn creation_date() -> DateTime<Utc> {
            "2022-08-22T01:02:19.824375631Z"
                .parse::<DateTime<Utc>>()
                .unwrap()
        }
        fn test_upload_no_expiry() -> Upload {
            Upload {
                id: String::from("abc123xyz"),
                filename: String::from("document.txt"),
                mimetype: "text/plain".parse().unwrap(),
                creation_date: creation_date(),
                expiry_date: None,
                delete_key: String::from("*^G^(MNCW#$(GMm9g87ctm4g98c43g789"),
            }
        }
        fn test_upload_expired() -> Upload {
            let mut upload = test_upload_no_expiry();
            upload.expiry_date = Some(
                "2022-08-29T01:02:19.824375631Z"
                    .parse::<DateTime<Utc>>()
                    .unwrap(),
            );
            upload
        }
        fn test_upload_not_expired() -> Upload {
            let mut upload = test_upload_expired();
            upload.expiry_date =
                Some(upload.expiry_date.unwrap() + TimeDelta::try_weeks(52000).unwrap());
            upload
        }

        #[test]
        fn no_expiry_is_not_expired() {
            assert!(!test_upload_no_expiry().is_expired());
        }
        #[test]
        fn not_expired() {
            assert!(!test_upload_not_expired().is_expired());
        }
        #[test]
        fn expired() {
            assert!(test_upload_expired().is_expired());
        }
    }
}

pub mod handle {
    //! Methods to create a handle (RAII guard) to interact with an upload
    //! stored on disk.
    //!
    //! NOTE: Currently you must call [`UploadHandle::flush`] since it can't do
    //! that automatically yet without an async [`Drop`] impl.
    use std::{io, path::PathBuf};

    use displaydoc::Display;
    use thiserror::Error;
    use tokio::{
        fs::{self, File},
        io::AsyncWriteExt,
    };

    use super::upload::Upload;
    use crate::serde::UploadMetadata;

    /// Make sure to call [`Self::flush`] or else the metadata won't be saved!
    // TODO: impl Drop so it can automatically flush() with RAII
    #[derive(Debug)]
    pub struct UploadHandle {
        /// path to the upload directory
        pub path: PathBuf,
        /// info about the upload, see [`Upload`]
        pub metadata: Upload,
        /// reference to the open uploaded file
        pub file: File,
        /// path of the uploaded file
        pub file_path: PathBuf,
        // marked pub(super) so it can be constructed by [`super::file`] methods
        pub(super) metadata_file: File,
        pub(super) lock_path: PathBuf,
    }
    /// Errors when flushing the upload metadata to disk
    #[derive(Debug, Error, Display)]
    pub enum FlushUploadError {
        /// error while serializing with serde_json
        Serialize(#[from] serde_json::Error),
        /// error writing metadata to file
        WriteMetadata(#[source] io::Error),

        /// error flushing metadata to disk
        FlushMetadata(#[source] io::Error),
        /// error flushing upload file to disk
        FlushFile(#[source] io::Error),

        /// error removing lock file
        RemoveLock(#[source] io::Error),
    }
    impl UploadHandle {
        /// Consume the handle, gracefully close the uploaded file, and flush
        /// the metadata to disk.
        pub async fn flush(mut self) -> Result<Upload, FlushUploadError> {
            self.metadata_file
                .write_all(
                    // TODO: get rid of self.metadata.clone()
                    serde_json::to_string(&UploadMetadata::from_upload(self.metadata.clone()))?
                        .as_bytes(),
                )
                .await
                .map_err(FlushUploadError::WriteMetadata)?;
            self.metadata_file
                .flush()
                .await
                .map_err(FlushUploadError::FlushMetadata)?;

            self.file
                .flush()
                .await
                .map_err(FlushUploadError::FlushFile)?;

            fs::remove_file(&self.lock_path)
                .await
                .map_err(FlushUploadError::RemoveLock)?;

            Ok(self.metadata)
        }

        /// Consume the handle, and just delete the lock file. Note that an
        /// invalid upload will be left behind until the next cleanup
        /// task.
        ///
        /// This is useful for handling a graceful shutdown.
        ///
        /// Also note that this is not done async because [`Drop`] can't be
        /// async yet.
        pub async fn drop_lock(self) -> Result<(), io::Error> {
            fs::remove_file(&self.lock_path).await
        }
    }
}
