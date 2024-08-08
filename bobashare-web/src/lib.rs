//! Webserver written with [`axum`] which provides a frontend and REST API for
//! [`bobashare`]

use std::{num::ParseIntError, sync::LazyLock, time::Duration as StdDuration};

use bobashare::storage::file::FileBackend;
use chrono::Duration;
use displaydoc::Display;
use pulldown_cmark::Options;
use regex::Regex;
use syntect::{html::ClassStyle, parsing::SyntaxSet};
use thiserror::Error;
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

/// Error encountered in converting string to duration values with [`str_to_duration`]
#[derive(Debug, Error, Display)]
pub enum StrToDurationError {
    /// string does not match duration format (try: 15d)
    Invalid,

    /// could not parse number in duration, is it too large?
    NumberParse(ParseIntError),
}

/// Regex used for duration string parsing in [`str_to_duration`]
static DURATION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9]+)(m|h|d|w|mon|y)$").unwrap());

/// Take a string with a simple duration format (single number followed by unit)
/// and output a [`StdDuration`]. Accepts durations in minutes (m), hours
/// (h), days (d), weeks (w), months (mon), or years (y).
///
/// A month is equivalent to 30 days. A year is equivalent to 365 days.
///
/// # Examples
///
/// Basic (small numbers that fit within the unit)
///
/// ```
/// use chrono::TimeDelta;
///
/// assert_eq!(
///     TimeDelta::from_std(bobashare_web::str_to_duration("17m").unwrap()).unwrap(),
///     TimeDelta::minutes(17)
/// );
/// assert_eq!(
///     TimeDelta::from_std(bobashare_web::str_to_duration("14h").unwrap()).unwrap(),
///     TimeDelta::hours(14)
/// );
/// assert_eq!(
///     TimeDelta::from_std(bobashare_web::str_to_duration("26d").unwrap()).unwrap(),
///     TimeDelta::days(26)
/// );
/// assert_eq!(
///     TimeDelta::from_std(bobashare_web::str_to_duration("2w").unwrap()).unwrap(),
///     TimeDelta::weeks(2)
/// );
/// assert_eq!(
///     TimeDelta::from_std(bobashare_web::str_to_duration("4mon").unwrap()).unwrap(),
///     TimeDelta::days(30 * 4)
/// );
/// assert_eq!(
///     TimeDelta::from_std(bobashare_web::str_to_duration("7y").unwrap()).unwrap(),
///     TimeDelta::days(365 * 7)
/// );
/// ```
///
/// Demonstrate the day values of months and years
///
/// ```
/// # use chrono::TimeDelta;
/// assert_eq!(
///     TimeDelta::from_std(bobashare_web::str_to_duration("1mon").unwrap()).unwrap(),
///     TimeDelta::days(30)
/// );
/// assert_eq!(
///     TimeDelta::from_std(bobashare_web::str_to_duration("1y").unwrap()).unwrap(),
///     TimeDelta::days(365)
/// );
/// ```
pub fn str_to_duration<S: AsRef<str>>(s: S) -> Result<StdDuration, StrToDurationError> {
    let caps = DURATION_REGEX
        .captures(s.as_ref())
        .ok_or(StrToDurationError::Invalid)?;
    let count = caps[1]
        .parse::<u64>()
        .map_err(StrToDurationError::NumberParse)?;
    Ok(match &caps[2] {
        "m" => StdDuration::from_secs(count * 60),
        "h" => StdDuration::from_secs(count * 60 * 60),
        "d" => StdDuration::from_secs(count * 60 * 60 * 24),
        "w" => StdDuration::from_secs(count * 60 * 60 * 24 * 7),
        "mon" => StdDuration::from_secs(count * 60 * 60 * 24 * 30),
        "y" => StdDuration::from_secs(count * 60 * 60 * 24 * 365),
        _ => panic!("invalid duration unit received from regex"),
    })
}
