//! Webserver written with [`axum`] which provides a frontend and REST API for
//! [`bobashare`]

use std::time::Duration as StdDuration;

use bobashare::storage::file::FileBackend;
use chrono::Duration;
use pulldown_cmark::Options;
use syntect::{html::ClassStyle, parsing::SyntaxSet};
use tokio::sync::broadcast;
use url::Url;

pub mod api;
pub mod static_routes;
pub mod views;

/// Prefix for CSS classes used for [`syntect`] highlighting
pub const HIGHLIGHT_CLASS_PREFIX: &str = "hl-";
/// [`ClassStyle`] used for [`syntect`] highlighting
pub const CLASS_STYLE: ClassStyle = ClassStyle::SpacedPrefixed {
    prefix: HIGHLIGHT_CLASS_PREFIX,
};

/// Options used for [`pulldown_cmark`] rendering
pub const MARKDOWN_OPTIONS: Options = Options::all();

/// A struct that contains all the state and config for bobashare
#[derive(Debug, Clone)]
pub struct AppState {
    /// storage backend
    pub backend: FileBackend,
    /// how often between each cleanup
    pub cleanup_interval: StdDuration,
    /// base URL (ex. `http://localhost:3000/`)
    pub base_url: Url,
    /// base URL for downloading raw upload files (ex. `http://localhost:3000/raw/`)
    pub raw_url: Url,
    /// length of randomly generated IDs
    pub id_length: usize,
    /// default expiry time
    pub default_expiry: Duration,
    /// maximum expiry time ([`None`] for no maximum)
    pub max_expiry: Option<Duration>,
    /// maximum file size in bytes
    pub max_file_size: u64,

    // syntax highlighting
    pub syntax_set: SyntaxSet,

    /// extra text to display in footer
    pub extra_footer_text: Option<String>,

    /// channel to broadcast shutdown -- will force all uploads to stop
    pub shutdown_tx: broadcast::Sender<()>,
}

/// Take the requested expiry, and make sure it's within the maximum expiry.
///
/// # Meaning of [`None`]
///
/// If the maximum expiry (`max_expiry`) is None, then any expiry will be
/// allowed, including no expiry. If the requested expiry (`other`) is
/// set to None, then it will return the maximum allowed expiry.
///
/// # Examples
///
/// Requesting no expiry with no maximum expiry:
///
/// ```
/// # use chrono::Duration;
/// let max_expiry = None;
/// assert_eq!(bobashare_web::clamp_expiry(max_expiry, None), None);
/// ```
///
/// Requesting no expiry but a maximum expiry is set (gives the maximum allowed
/// expiry):
///
/// ```
/// # use chrono::Duration;
/// let max_expiry = Some(Duration::days(7));
/// assert_eq!(bobashare_web::clamp_expiry(max_expiry, None), max_expiry);
/// ```
///
/// Requesting an expiry with no maximum expiry:
///
/// ```
/// # use chrono::Duration;
/// let max_expiry = None;
/// assert_eq!(
///     bobashare_web::clamp_expiry(max_expiry, Some(Duration::days(3))),
///     Some(Duration::days(3)),
/// );
/// ```
///
/// Requesting an expiry that's within the maximum expiry:
///
/// ```
/// # use chrono::Duration;
/// let max_expiry = Some(Duration::days(7));
/// assert_eq!(
///     bobashare_web::clamp_expiry(max_expiry, Some(Duration::days(3))),
///     Some(Duration::days(3)),
/// );
/// ```
///
/// Requesting an expiry that's outside of the maximum expiry (clamps to the
/// maximum expiry):
///
/// ```
/// # use chrono::Duration;
/// let max_expiry = Some(Duration::days(7));
/// assert_eq!(
///     bobashare_web::clamp_expiry(max_expiry, Some(Duration::days(30))),
///     max_expiry,
/// );
/// ```
pub fn clamp_expiry(max_expiry: Option<Duration>, other: Option<Duration>) -> Option<Duration> {
    match other {
        // if no expiry requested, use the max no matter what
        None => max_expiry,
        Some(e) => match max_expiry {
            // if no max expiry, keep requested expiry
            None => Some(e),
            Some(max) => Some(e.clamp(Duration::zero(), max)),
        },
    }
}
