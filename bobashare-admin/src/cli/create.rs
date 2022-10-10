use std::path::PathBuf;

use anyhow::{anyhow, Context};
use bobashare::{generate_randomized_id, storage::file::FileBackend};
use chrono::Duration;
use clap::{Args, Subcommand};
use tokio::{
    fs::File,
    io::{self, AsyncWriteExt},
};
use tracing::{event, instrument, Level};

#[derive(Debug, Args, Clone)]
pub(crate) struct CreateUpload {
    /// How long (in days) before the upload expires and is deleted.
    ///
    /// If not provided, the default is no expiry (permanent).
    #[clap(short, long, value_parser)]
    expiry: Option<u16>,
    #[clap(short, long, value_parser)]
    source_file: PathBuf,

    #[clap(subcommand)]
    name: NameOptions,
}
#[derive(Debug, Subcommand, Clone)]
pub(crate) enum NameOptions {
    /// Use a randomized name for the upload
    Random {
        /// The length of name to randomly generate
        #[clap(short, long, default_value_t = 8)]
        length: u16,
    },
    /// Use a specific name for the upload
    Name {
        #[clap(short, long)]
        name: String,
    },
}

#[instrument]
pub(crate) async fn create_upload(backend: FileBackend, args: CreateUpload) -> anyhow::Result<()> {
    let expiry = args.expiry.map(|e| Duration::days(e.into()));
    let name = match args.name {
        // TODO: handle already existing name
        NameOptions::Name { name } => name,
        NameOptions::Random { length } => generate_randomized_id(length.into()),
    };

    let filename = args
        .source_file
        .file_name()
        .ok_or_else(|| anyhow!("invalid filename for source file"))?
        .to_string_lossy()
        .to_string();
    let mut file = File::open(&args.source_file)
        .await
        .with_context(|| format!("error opening file at {:?}", &args.source_file))?;
    let mimetype = mime_guess::from_path(&args.source_file).first_or_octet_stream();

    let mut upload = backend
        .create_upload(name, filename, mimetype, expiry)
        .await?;

    println!("{:?}", upload.metadata);

    let copied = io::copy(&mut file, &mut upload.file).await?;

    event!(Level::DEBUG, "Wrote {} bytes to the upload file", copied);
    upload.file.flush().await?;
    upload.flush().await?;

    Ok(())
}
