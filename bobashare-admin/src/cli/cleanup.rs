use bobashare::storage::file::FileBackend;
use clap::Args;
use tracing::instrument;

#[derive(Debug, Clone, Args)]
pub(crate) struct Cleanup {}

#[instrument(skip(backend))]
pub(crate) async fn cleanup(backend: FileBackend, args: Cleanup) -> anyhow::Result<()> {
    backend.cleanup().await?;
    Ok(())
}
