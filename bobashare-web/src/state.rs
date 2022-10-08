use bobashare::storage::file::FileBackend;
use chrono::Duration;
use hyper::Uri;

/// A struct that contains all the state and config for bobashare
#[derive(Debug)]
pub struct AppState {
    pub backend: FileBackend,
    pub root_url: Uri,
    pub url_length: usize,
    pub default_expiry: Duration,
    pub max_expiry: Option<Duration>,
}

impl AppState {
    pub fn join_url(&self, other: String) -> Uri {
        todo!("{}", other)
    }
}
