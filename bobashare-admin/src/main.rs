//! Admin CLI for managing bobashare

use std::path::PathBuf;

use anyhow::Context;
use bobashare::storage::file::FileBackend;
// use chrono::{prelude::*, Duration};
use clap::{Parser, Subcommand};
use cli::create::CreateUpload;

pub(crate) mod cli;

#[derive(Debug, Parser)]
pub(crate) struct Cli {
    #[clap(short, long, value_parser, default_value = "experiment/")]
    root: PathBuf,
    #[clap(subcommand)]
    command: Command,
}
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    CreateUpload(CreateUpload),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let backend = FileBackend::new(cli.root)
        .await
        .context("error creating file backend")?;

    match cli.command {
        Command::CreateUpload(args) => {
            cli::create::create_upload(backend, args).await?;
        }
    };

    Ok(())
}
