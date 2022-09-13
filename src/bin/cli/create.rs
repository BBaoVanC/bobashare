use anyhow::Context;
use bobashare::backend::{
    generate_randomized_name,
    storage::file::{CreateUploadError, FileBackend},
};
use chrono::Duration;
use clap::{Args, Subcommand};
use log::info;

// #[derive(Debug, Args)]
// pub(crate) struct CreateUpload {
//     expiry: Option<u16>,
//     #[clap(subcommand)]
//     name: NameOptions,
// }

// #[derive(Debug, Subcommand)]
// pub(crate) enum NameOptions {
//     Randomized,
//     Name {
//         #[clap(short, long, value_parser)]
//         name: String,
//     },
// }

#[derive(Debug, Args, Clone)]
// #[clap(group(
//     ArgGroup::new("random_or_name").required(true).args(&["random", "name"])
// ))]
pub(crate) struct CreateUpload {
    #[clap(short, long, value_parser)]
    /// How long (in days) before the upload expires and is deleted.
    /// 
    /// If not provided, the default is no expiry (permanent).
    expiry: Option<u16>,

    #[clap(subcommand)]
    name: NameOptions
    // #[clap(short, long)]
    // random: bool,
    // #[clap(short, long, value_parser)]
    // name: Option<String>,
}
#[derive(Debug, Subcommand, Clone)]
pub(crate) enum NameOptions {
    /// Use a randomized name for the upload
    Random {
        /// The length of name to randomly generate
        length: u16,
    },
    /// Use a specific name for the upload
    Name {
        name: String,
    },
}

pub(crate) async fn create_upload(backend: FileBackend, args: CreateUpload) -> anyhow::Result<()> {
    let expiry = args.expiry.map(|e| Duration::days(e.into()));

    let upload = match args.name {
        NameOptions::Random{ length } => {
            loop {
                let name = generate_randomized_name(length.into());
                let res = backend.create_upload(&name, expiry).await;
                if let Err(CreateUploadError::AlreadyExists) = res {
                    // TODO: should use tracing
                    info!("An upload with the randomized name {} already exists; trying a new name", &name);
                    continue;
                }
                break res.context("error creating upload")?
            }
        },
        NameOptions::Name { name } => {
            backend.create_upload(name, expiry).await.context("error creating upload")?
        }
    };

    println!("{:?}", upload);

    Ok(())
}
