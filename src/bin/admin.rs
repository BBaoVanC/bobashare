//! Admin CLI for managing bobashare

use std::path::PathBuf;

use anyhow::Context;
use bobashare::backend::storage::file::FileBackend;
use chrono::{prelude::*, Duration};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long, value_parser, default_value = "experiment/")]
    root: PathBuf,
    #[clap(subcommand)]
    command: Command,
}
#[derive(Debug, Subcommand)]
enum Command {
    CreateUpload(CreateUpload),
}

#[derive(Debug, Args)]
struct CreateUpload {
    #[clap(short, long, value_parser)]
    /// The name of the upload, which will be its URL
    //
    // TODO: If not provided, default is randomly generated
    name: String,
    #[clap(short, long, value_parser)]
    /// How long (in days) the upload should stay before it expires and is
    /// deleted
    ///
    /// If not provided, it defaults to no expiry (permanent)
    expiry: Option<u16>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let backend = FileBackend::new(cli.root)
        .await
        .context("error creating file backend")?;

    match cli.command {
        Command::CreateUpload(args) => {
            let upload = backend
                .create_upload(args.name, args.expiry.map(|e| Duration::days(e.into())))
                .await?;
            if let Some(expiry) = upload.expiry_date {
                println!("{}", expiry - upload.creation_date);
            } else {
                println!("doesn't expire");
            }
        }
    };

    Ok(())
}
