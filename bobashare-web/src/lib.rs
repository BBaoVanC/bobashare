//! Webserver written with [`axum`] which provides a frontend and REST API for
//! [`bobashare`]

use std::{num::ParseIntError, path::PathBuf, str::FromStr, time::Duration as StdDuration};

use bobashare::storage::file::FileBackend;
use chrono::TimeDelta;
use displaydoc::Display;
use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
};
use thiserror::Error;
use tokio::sync::broadcast;
use tracing::{event, instrument, Level};
use url::Url;

pub mod api;
pub mod static_routes;
pub mod views;

#[cfg(test)]
mod tests;

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
    /// instance name, displayed on all pages
    pub instance_name: String,
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
    pub default_expiry: TimeDelta,
    /// maximum expiry time ([`None`] for no maximum)
    pub max_expiry: Option<TimeDelta>,
    /// maximum file size in bytes
    pub max_file_size: u64,

    // syntax highlighting
    pub syntax_set: SyntaxSet,

    /// extra text to display in footer
    pub extra_footer_text: Option<String>,
    /// path to markdown file for about page
    pub about_page: Option<PathBuf>,
    /// raw markdown text content of about page file
    pub about_page_content: String,

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
/// # use chrono::TimeDelta;
/// let max_expiry = None;
/// assert_eq!(bobashare_web::clamp_expiry(max_expiry, None), None);
/// ```
///
/// Requesting no expiry but a maximum expiry is set (gives the maximum allowed
/// expiry):
///
/// ```
/// # use chrono::TimeDelta;
/// let max_expiry = Some(TimeDelta::days(7));
/// assert_eq!(bobashare_web::clamp_expiry(max_expiry, None), max_expiry);
/// ```
///
/// Requesting an expiry with no maximum expiry:
///
/// ```
/// # use chrono::TimeDelta;
/// let max_expiry = None;
/// assert_eq!(
///     bobashare_web::clamp_expiry(max_expiry, Some(TimeDelta::days(3))),
///     Some(TimeDelta::days(3)),
/// );
/// ```
///
/// Requesting an expiry that's within the maximum expiry:
///
/// ```
/// # use chrono::TimeDelta;
/// let max_expiry = Some(TimeDelta::days(7));
/// assert_eq!(
///     bobashare_web::clamp_expiry(max_expiry, Some(TimeDelta::days(3))),
///     Some(TimeDelta::days(3)),
/// );
/// ```
///
/// Requesting an expiry that's outside of the maximum expiry (clamps to the
/// maximum expiry):
///
/// ```
/// # use chrono::TimeDelta;
/// let max_expiry = Some(TimeDelta::days(7));
/// assert_eq!(
///     bobashare_web::clamp_expiry(max_expiry, Some(TimeDelta::days(30))),
///     max_expiry,
/// );
/// ```
pub fn clamp_expiry(max_expiry: Option<TimeDelta>, other: Option<TimeDelta>) -> Option<TimeDelta> {
    match other {
        // if no expiry requested, use the max no matter what
        None => max_expiry,
        Some(e) => match max_expiry {
            // if no max expiry, keep requested expiry
            None => Some(e),
            Some(max) => Some(e.clamp(TimeDelta::zero(), max)),
        },
    }
}

/// Error encountered in converting string to duration values with
/// [`str_to_duration`]
#[derive(Debug, Error, Display)]
pub enum StrToDurationError {
    /// string does not match duration format (try: 15d)
    Invalid,

    /// could not parse number in duration, is it too large?
    NumberParse(#[from] ParseIntError),
}

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
/// use bobashare_web::str_to_duration;
/// use chrono::TimeDelta;
///
/// assert_eq!(
///     TimeDelta::from_std(str_to_duration("17m")?)?,
///     TimeDelta::minutes(17),
/// );
/// assert_eq!(
///     TimeDelta::from_std(str_to_duration("14h")?)?,
///     TimeDelta::hours(14),
/// );
/// assert_eq!(
///     TimeDelta::from_std(str_to_duration("26d")?)?,
///     TimeDelta::days(26),
/// );
/// assert_eq!(
///     TimeDelta::from_std(str_to_duration("2w")?)?,
///     TimeDelta::weeks(2),
/// );
/// assert_eq!(
///     TimeDelta::from_std(str_to_duration("4mon")?)?,
///     TimeDelta::days(30 * 4),
/// );
/// assert_eq!(
///     TimeDelta::from_std(str_to_duration("7y")?)?,
///     TimeDelta::days(365 * 7),
/// );
///
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// Demonstrate the day values of months and years
///
/// ```
/// # use bobashare_web::str_to_duration;
/// # use chrono::TimeDelta;
/// assert_eq!(
///     TimeDelta::from_std(str_to_duration("1mon")?)?,
///     TimeDelta::days(30),
/// );
/// assert_eq!(
///     TimeDelta::from_std(str_to_duration("1y")?)?,
///     TimeDelta::days(365),
/// );
/// # Ok::<(), anyhow::Error>(())
/// ```
// TODO: make it look nicer
pub fn str_to_duration(s: &str) -> Result<StdDuration, StrToDurationError> {
    let mut chars = s.char_indices();
    if !chars.next().is_some_and(|(_, c)| c.is_ascii_digit()) {
        return Err(StrToDurationError::Invalid);
    }

    let unit_idx = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());

    let count_str = &s[..unit_idx];
    let count = u64::from_str(count_str)?;
    let unit_str = &s[unit_idx..];

    Ok(match unit_str {
        "s" => StdDuration::from_secs(count),
        "m" => StdDuration::from_secs(count * 60),
        "h" => StdDuration::from_secs(count * 60 * 60),
        "d" => StdDuration::from_secs(count * 60 * 60 * 24),
        "w" => StdDuration::from_secs(count * 60 * 60 * 24 * 7),
        "mon" => StdDuration::from_secs(count * 60 * 60 * 24 * 30),
        "y" => StdDuration::from_secs(count * 60 * 60 * 24 * 365),
        _ => return Err(StrToDurationError::Invalid),
    })
}

#[derive(Debug, Error, Display)]
/// Errors for [`render_markdown_with_syntax_set`]
pub enum RenderMarkdownWithSyntaxError {
    /// error highlighting markdown-fenced code block: {0}
    HighlightCodeBlock(#[source] syntect::Error),

    /// error rendering math with katex: {0}
    RenderMath(#[source] katex::Error),
}

/// Render markdown into HTML, including syntax highlighting for code blocks
/// using [`syntect`].
///
/// Takes in a [`SyntaxSet`] to use for highlighting.
#[instrument(name = "render", skip(syntax_set, source), level = "debug")]
pub fn render_markdown_with_syntax_set(
    source: &str,
    syntax_set: &SyntaxSet,
) -> Result<String, RenderMarkdownWithSyntaxError> {
    let mut parser = Parser::new_ext(source, MARKDOWN_OPTIONS).peekable();
    let mut output = Vec::new();
    // wrap multiline code blocks in a pre.highlight, and apply a syntect class
    // to the inner code
    while let Some(event) = parser.next() {
        match event {
            // patch GHSA-g7gw-4888-mr65
            Event::Html(s) | Event::InlineHtml(s) => {
                event!(Level::TRACE, ?s, "removed raw HTML");
            }

            Event::InlineMath(s) => {
                let rendered =
                    katex::render(&s).map_err(RenderMarkdownWithSyntaxError::RenderMath)?;
                output.push(Event::Html(rendered.into()));
            }
            Event::DisplayMath(s) => {
                let opts = katex::Opts::builder().display_mode(true).build().unwrap();
                let rendered = katex::render_with_opts(&s, opts)
                    .map_err(RenderMarkdownWithSyntaxError::RenderMath)?;
                output.push(Event::Html(rendered.into()));
            }

            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(token))) => {
                output.push(Event::Html("<pre class=\"highlight\">".into()));
                let syntax = syntax_set
                    .find_syntax_by_token(&token)
                    .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
                let mut generator =
                    ClassedHTMLGenerator::new_with_class_style(syntax, syntax_set, CLASS_STYLE);

                // peek so we don't consume the end tag
                // TODO: figure out if take_while() can do this better
                while let Some(Event::Text(t)) = parser.peek() {
                    generator
                        .parse_html_for_line_which_includes_newline(t)
                        .map_err(RenderMarkdownWithSyntaxError::HighlightCodeBlock)?;
                    parser.next();
                }
                output.push(Event::Html(generator.finalize().into()));
            }
            Event::End(TagEnd::CodeBlock) => {
                output.push(Event::Html("</pre>".into()));
            }
            e => output.push(e),
        }
    }

    // FIXME: figure out where this specific calculation came from
    let mut displayed = String::with_capacity(source.len() * 3 / 2);
    push_html(&mut displayed, output.into_iter());
    Ok(displayed)
}
