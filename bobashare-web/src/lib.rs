//! Webserver written with [`axum`] which provides a frontend and REST API for
//! [`bobashare`]

use bobashare::storage::file::FileBackend;
use chrono::Duration;
use syntect::parsing::SyntaxSet;
use url::Url;

pub mod api;
pub mod static_routes;
pub mod views;

/// A struct that contains all the state and config for bobashare
#[derive(Debug, Clone)]
pub struct AppState {
    /// storage backend
    pub backend: FileBackend,
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

    // syntax highlighting
    pub syntax_set: SyntaxSet,
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

// TODO: document
pub fn iter_expiries() -> impl Iterator<Item = (Duration, &'static str, &'static str)> {
    [
        (Duration::minutes(5), "5m", "5 minutes"),
        (Duration::minutes(10), "10m", "10 minutes"),
        (Duration::minutes(30), "30m", "30 minutes"),
        (Duration::hours(1), "1h", "1 hour"),
        (Duration::hours(2), "2h", "2 hours"),
        (Duration::hours(6), "6h", "6 hours"),
        (Duration::hours(12), "12h", "12 hours"),
        (Duration::days(1), "1d", "1 day"),
        (Duration::days(2), "2d", "2 days"),
        (Duration::weeks(1), "1w", "1 week"),
        (Duration::weeks(2), "2w", "2 weeks"),
        (Duration::days(30), "1mon", "1 month"),
        (Duration::days(60), "2mon", "2 months"),
        (Duration::days(90), "3mon", "3 months"),
        (Duration::days(180), "6mon", "6 months"),
        (Duration::days(365), "1y", "1 year"),
    ].into_iter()
}
