//! Routes to display or download an upload in a browser

use std::sync::Arc;

use anyhow::Context;
use askama::Template;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use bobashare::storage::{file::OpenUploadError, handle::UploadHandle};
use chrono::{DateTime, Duration, Utc};
use displaydoc::Display;
use hyper::{header, StatusCode};
use mime::Mime;
use pulldown_cmark::{html::push_html, CodeBlockKind, Event, Parser, Tag, TagEnd};
use serde::{Deserialize, Deserializer};
use syntect::{html::ClassedHTMLGenerator, util::LinesWithEndings};
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio_util::io::ReaderStream;
use tracing::{event, instrument, Level};
use url::Url;

use super::{filters, prelude::*, render_template, ErrorResponse, ErrorTemplate, TemplateState};
use crate::{AppState, CLASS_STYLE, MARKDOWN_OPTIONS};

/// Errors when trying to view/download an upload
#[derive(Debug, Error, Display)]
pub enum ViewUploadError {
    /// an upload at the specified id was not found
    NotFound,

    /// internal server error
    InternalServer(#[from] anyhow::Error),
}
impl From<OpenUploadError> for ViewUploadError {
    fn from(err: OpenUploadError) -> Self {
        match err {
            OpenUploadError::NotFound(_) => Self::NotFound,
            _ => Self::InternalServer(anyhow::Error::new(err).context("error opening upload")),
        }
    }
}

async fn open_upload<S: AsRef<str>>(
    state: &AppState,
    id: S,
) -> Result<UploadHandle, ViewUploadError> {
    let upload = state.backend.open_upload(id.as_ref(), false).await?;

    if upload.metadata.is_expired() {
        event!(Level::INFO, "upload is expired; it will be deleted");
        // don't upload.flush() since it's not open for writing -- it will fail
        state
            .backend
            .delete_upload(id.as_ref())
            .await
            .context("error deleting expired upload")?;
        return Err(ViewUploadError::NotFound);
    }

    Ok(upload)
}

#[derive(Template)]
#[template(path = "display.html.jinja")]
pub struct DisplayTemplate<'s> {
    pub state: TemplateState<'s>,
    pub id: String,
    pub filename: String,
    pub expiry_date: Option<DateTime<Utc>>,
    pub expiry_relative: Option<Duration>,
    pub size: u64,
    pub mimetype: Mime,
    pub contents: DisplayType,
    pub raw_url: Url,
    pub download_url: Url,
}
#[derive(Debug)]
pub enum DisplayType {
    Text {
        highlighted: String,
    },
    Markdown {
        highlighted: String,
        displayed: String,
    },
    Image,
    Video,
    Audio,
    Pdf,
    Other,
    TooLarge,
}

/// Maximum file size that will be rendered
const MAX_DISPLAY_SIZE: u64 = 1024 * 1024; // 1 MiB

/// Display an upload as HTML
#[instrument(skip(state))]
pub async fn display(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let tmpl_state = TemplateState::from(&*state);
    let mut upload = open_upload(&state, id).await.map_err(|e| match e {
        ViewUploadError::NotFound => ErrorTemplate {
            state: tmpl_state.clone(),
            code: StatusCode::NOT_FOUND,
            message: e.to_string(),
        },
        ViewUploadError::InternalServer(_) => ErrorTemplate {
            state: tmpl_state.clone(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        },
    })?;
    let size = upload
        .file
        .metadata()
        .await
        .map_err(|e| ErrorTemplate {
            state: tmpl_state.clone(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("error reading file size: {e}"),
        })?
        .len();

    let contents = {
        let mimetype = upload.metadata.mimetype.clone();
        match (mimetype.type_(), mimetype.subtype()) {
            (mime::TEXT, _) | (mime::APPLICATION, mime::JSON) => {
                if size > MAX_DISPLAY_SIZE {
                    DisplayType::TooLarge
                } else {
                    let extension = std::path::Path::new(&upload.metadata.filename)
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    let syntax = state
                        .syntax_set
                        .find_syntax_by_extension(extension)
                        .unwrap_or_else(|| state.syntax_set.find_syntax_plain_text());
                    let mut contents = String::with_capacity(size.try_into().unwrap_or(usize::MAX));
                    upload
                        .file
                        .read_to_string(&mut contents)
                        .await
                        .map_err(|e| ErrorTemplate {
                            state: tmpl_state.clone(),
                            code: StatusCode::INTERNAL_SERVER_ERROR,
                            message: format!("error reading file contents: {e}"),
                        })?;

                    event!(
                        Level::DEBUG,
                        "highlighting file with syntax {}",
                        syntax.name
                    );
                    let highlighted = {
                        let mut generator = ClassedHTMLGenerator::new_with_class_style(
                            syntax,
                            &state.syntax_set,
                            CLASS_STYLE,
                        );
                        for line in LinesWithEndings::from(&contents) {
                            generator
                                .parse_html_for_line_which_includes_newline(line)
                                .map_err(|e| ErrorTemplate {
                                    state: tmpl_state.clone(),
                                    code: StatusCode::INTERNAL_SERVER_ERROR,
                                    message: format!("error highlighting file contents: {e}"),
                                })?;
                        }
                        generator.finalize()
                    };

                    if extension.eq_ignore_ascii_case("md") {
                        let mut parser = Parser::new_ext(&contents, MARKDOWN_OPTIONS).peekable();
                        let mut output = Vec::new();
                        while let Some(event) = parser.next() {
                            match event {
                                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(token))) => {
                                    output.push(Event::Html("<pre class=\"highlight\">".into()));
                                    let syntax = state
                                        .syntax_set
                                        .find_syntax_by_token(&token)
                                        .unwrap_or_else(|| {
                                            state.syntax_set.find_syntax_plain_text()
                                        });
                                    let mut generator = ClassedHTMLGenerator::new_with_class_style(
                                        syntax,
                                        &state.syntax_set,
                                        CLASS_STYLE,
                                    );

                                    // peek so we don't consume the end tag
                                    // TODO: figure out if take_while() can do this better
                                    while let Some(Event::Text(t)) = parser.peek() {
                                        generator
                                            .parse_html_for_line_which_includes_newline(t)
                                            .map_err(|e| ErrorTemplate {
                                                state: tmpl_state.clone(),
                                                code: StatusCode::INTERNAL_SERVER_ERROR,
                                                message: format!(
                                                    "error highlighting markdown fenced code block: {e}",
                                                ),
                                            })?;
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

                        let mut displayed = String::with_capacity(contents.len() * 3 / 2);
                        push_html(&mut displayed, output.into_iter());

                        DisplayType::Markdown {
                            highlighted,
                            displayed,
                        }
                    } else {
                        DisplayType::Text { highlighted }
                    }
                }
            }
            (mime::IMAGE, _) => DisplayType::Image,
            (mime::VIDEO, _) => DisplayType::Video,
            (mime::AUDIO, _) => DisplayType::Audio,
            (mime::APPLICATION, mime::PDF) => DisplayType::Pdf,
            (_, _) => DisplayType::Other,
        }
    };

    event!(Level::DEBUG, "rendering upload template");
    let raw_url = state.raw_url.join(&upload.metadata.id).unwrap();
    let mut download_url = raw_url.clone();
    download_url.set_query(Some("download"));
    render_template(DisplayTemplate {
        raw_url,
        download_url,
        id: upload.metadata.id,
        filename: upload.metadata.filename,
        expiry_date: upload.metadata.expiry_date,
        expiry_relative: upload.metadata.expiry_date.map(|e| e - Utc::now()),
        size,
        mimetype: upload.metadata.mimetype,
        contents,
        state: tmpl_state,
    })
}

fn string_is_true<'de, D>(_: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(true)
}
#[derive(Debug, Deserialize)]
pub struct RawParams {
    #[serde(default, deserialize_with = "string_is_true")]
    download: bool,
}
/// Download the raw upload file
#[instrument(skip(state))]
pub async fn raw(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(RawParams { download }): Query<RawParams>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let tmpl_state = TemplateState::from(&*state);
    let upload = open_upload(&state, id).await.map_err(|e| match e {
        ViewUploadError::NotFound => ErrorTemplate {
            state: tmpl_state.clone(),
            code: StatusCode::NOT_FOUND,
            message: e.to_string(),
        },
        ViewUploadError::InternalServer(_) => ErrorTemplate {
            state: tmpl_state.clone(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: e.to_string(),
        },
    })?;

    let size = upload
        .file
        .metadata()
        .await
        .map_err(|e| ErrorTemplate {
            state: tmpl_state.clone(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("error reading file size: {e}"),
        })?
        .len();
    event!(Level::DEBUG, size, "found size of upload file",);

    let body = Body::from_stream(ReaderStream::new(upload.file));

    event!(
        Level::INFO,
        "type" = %upload.metadata.mimetype,
        length = size,
        filename = upload.metadata.filename,
        "successfully streaming upload file to client"
    );
    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, upload.metadata.mimetype.to_string()),
            (header::CONTENT_LENGTH, size.to_string()),
            (
                header::CONTENT_DISPOSITION,
                // if params.download {
                if download {
                    format!("attachment; filename=\"{}\"", upload.metadata.filename)
                } else {
                    format!("inline; filename=\"{}\"", upload.metadata.filename)
                },
            ),
        ],
        body,
    ))
}
