//! Admin CLI for managing bobashare

use std::path::PathBuf;

use anyhow::Context;
use bobashare::backend::storage::file::FileBackend;
use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long, value_parser)]
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
    name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let backend = FileBackend::new(cli.root).await.context("error creating file backend")?;


    Ok(())
}
