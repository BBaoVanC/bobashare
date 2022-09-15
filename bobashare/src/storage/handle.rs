#[derive(Debug)]
pub struct UploadFileHandle<'h> {
    pub metadata: UploadFile,
    pub file: File,
    files_vec: &'h mut Vec<UploadFile>,
}
impl<'h> UploadFileHandle<'_> {
    pub fn new(
        metadata: UploadFile,
        file: File,
        files_vec: &'h mut Vec<UploadFile>,
    ) -> UploadFileHandle<'h> {
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
