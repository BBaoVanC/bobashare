//! Frontend views (as opposed to REST API)

use std::sync::Arc;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use chrono::Duration;
use hyper::StatusCode;
use tracing::{event, Level};
use url::Url;

use crate::AppState;

pub mod display;
pub mod filters;
pub mod upload;

mod prelude {
    pub use super::CurrentNavigation;
}

#[derive(Debug, Clone)]
pub struct TemplateState {
    version: &'static str,
    base_url: Url,
    max_file_size: u64,
    max_expiry: Option<Duration>,
    extra_footer_text: Option<String>,

    // None if the current page is not a navbar item
    current_navigation: Option<CurrentNavigation>,
}
impl From<&AppState> for TemplateState {
    fn from(state: &AppState) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
            base_url: state.base_url.clone(),
            max_file_size: state.max_file_size,
            max_expiry: state.max_expiry,
            extra_footer_text: state.extra_footer_text.clone(),
            current_navigation: None, // will be set to Some in individual handlers
        }
    }
}
impl From<Arc<AppState>> for TemplateState {
    fn from(state: Arc<AppState>) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
            base_url: state.base_url.clone(),
            max_file_size: state.max_file_size,
            max_expiry: state.max_expiry,
            extra_footer_text: state.extra_footer_text.clone(),
            current_navigation: None,
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

// which page is current navigated to, for navbar formatting
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum CurrentNavigation {
    Upload,
    Paste,
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
        let code = self.0.code;
        let error_msg = &self.0.message;
        if code.is_server_error() {
            event!(Level::ERROR, status = code.as_u16(), error = error_msg);
        } else if code.is_client_error() {
            event!(Level::WARN, status = code.as_u16(), error = error_msg);
        } else {
            event!(Level::INFO, status = code.as_u16(), error = error_msg);
        }

        (self.0.code, self.0).into_response()
    }
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(upload::upload))
        .route("/paste/", get(upload::paste))
        .route("/:id", get(display::display))
        .route("/raw/:id", get(display::raw))
}
