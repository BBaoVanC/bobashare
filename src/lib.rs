//! Simple ephemeral file sharing service

use backend::storage::file::FileBackend;

pub mod backend;

pub struct AppState {
    pub backend: FileBackend,
}
