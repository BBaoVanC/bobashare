//! Admin CLI for managing bobashare

use std::path::PathBuf;

use anyhow::Context;
use bobashare::storage::file::FileBackend;
use clap::{Parser, Subcommand};
use cli::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) mod cli;

#[derive(Debug, Clone, Parser)]
pub(crate) struct Cli {
    #[clap(short, long, value_parser, default_value = "storage/")]
    root: PathBuf,
    #[clap(subcommand)]
    command: Command,
}
#[derive(Debug, Clone, Subcommand)]
pub(crate) enum Command {
    CreateUpload(create::CreateUpload),
    Cleanup(cleanup::Cleanup),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TODO: set up logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug,bobashare=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    let backend = FileBackend::new(cli.root)
        .await
        .context("error creating file backend")?;

    match cli.command {
        Command::CreateUpload(args) => {
            cli::create::create_upload(backend, args).await?;
        },
        Command::Cleanup(args) => {
            cli::cleanup::cleanup(backend, args).await?;
        },
    };

    Ok(())
}
