//! Simple ephemeral file sharing service

use backend::storage::FileBackend;

pub mod api;
pub mod backend;

pub struct AppState {
    pub backend: FileBackend,
}
