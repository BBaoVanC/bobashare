//! Frontend views (as opposed to REST API)

use std::{collections::HashMap, sync::Arc};

use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use chrono::Duration;
use comrak::adapters::SyntaxHighlighterAdapter;
use hyper::StatusCode;
use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};
use url::Url;

use crate::AppState;

pub mod about;
pub mod display;
pub mod filters;
pub mod upload;

#[derive(Debug, Clone)]
pub struct TemplateState {
    base_url: Url,
    max_file_size: u64,
    max_expiry: Option<Duration>,
    extra_footer_text: Option<String>,
}
impl From<&AppState> for TemplateState {
    fn from(state: &AppState) -> Self {
        Self {
            base_url: state.base_url.clone(),
            max_file_size: state.max_file_size,
            max_expiry: state.max_expiry,
            extra_footer_text: state.extra_footer_text.clone(),
        }
    }
}
impl From<Arc<AppState>> for TemplateState {
    fn from(state: Arc<AppState>) -> Self {
        Self {
            base_url: state.base_url.clone(),
            max_file_size: state.max_file_size,
            max_expiry: state.max_expiry,
            extra_footer_text: state.extra_footer_text.clone(),
        }
    }
}
impl TemplateState {
    pub fn icon<S: AsRef<str>>(&self, name: S) -> Url {
        self.base_url
            .join(&format!("static/svg/fa/{}.svg", name.as_ref()))
            .unwrap()
    }
}

#[derive(Template)]
#[template(path = "error.html.jinja")]
pub struct ErrorTemplate {
    pub state: TemplateState,
    pub code: StatusCode,
    pub message: String,
}

pub struct ErrorResponse(pub ErrorTemplate);
impl From<ErrorTemplate> for ErrorResponse {
    fn from(template: ErrorTemplate) -> Self {
        Self(template)
    }
}
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> askama_axum::Response {
        (self.0.code, self.0).into_response()
    }
}

// TODO: find a better location for this
pub struct SyntectHighlighter {
    syntax_set: SyntaxSet,
    class_style: ClassStyle,
}
impl SyntectHighlighter {
    pub fn new(syntax_set: SyntaxSet, class_style: ClassStyle) -> Self {
        Self {
            syntax_set,
            class_style,
        }
    }
}
impl SyntaxHighlighterAdapter for SyntectHighlighter {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        let syntax = if let Some(s) = lang {
            self.syntax_set.find_syntax_by_token(s)
        } else {
            None
        }
        .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let mut generator =
            ClassedHTMLGenerator::new_with_class_style(syntax, &self.syntax_set, self.class_style);
        for line in LinesWithEndings::from(code) {
            // TODO: handle errors
            generator
                .parse_html_for_line_which_includes_newline(line)
                .unwrap();
        }
        let output = generator.finalize();
        // TODO: regex maybe (https://docs.rs/comrak/latest/src/comrak/plugins/syntect.rs.html#40)
        output.replace("<pre>", "").replace("</pre>", "")
    }
    fn build_pre_tag(&self, attributes: &HashMap<String, String>) -> String {
        let mut output = String::from("<pre class=\"highlight\"");
        for (key, value) in attributes {
            output.push(' ');
            output.push_str(key);
            output.push('=');
            output.push('"');
            output.push_str(value);
            output.push('"');
        }
        output.push('>');
        output
    }
    fn build_code_tag(&self, attributes: &HashMap<String, String>) -> String {
        let mut output = String::from("<code");
        for (key, value) in attributes {
            output.push(' ');
            output.push_str(key);
            output.push('=');
            output.push('"');
            output.push_str(value);
            output.push('"');
        }
        output.push('>');
        output
    }
}

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(state)
        .route("/about/", get(about::about))
        .route("/", get(upload::upload))
        .route("/paste/", get(upload::paste))
        .route("/:id", get(display::display))
        .route("/raw/:id", get(display::raw))
}
