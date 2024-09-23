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

// 's is for &AppState
#[derive(Debug, Clone)]
pub struct TemplateState<'s> {
    version: &'static str,
    base_url: &'s Url,
    max_file_size: u64,
    max_expiry: Option<Duration>,
    extra_footer_text: Option<&'s str>,

    // None if the current page is not a navbar item
    current_navigation: Option<CurrentNavigation>,
}
impl<'s> From<&'s AppState> for TemplateState<'s> {
    fn from(state: &'s AppState) -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
            base_url: &state.base_url,
            max_file_size: state.max_file_size,
            max_expiry: state.max_expiry,
            extra_footer_text: state.extra_footer_text.as_deref(),
            current_navigation: None, // will be set to Some in individual handlers
        }
    }
}
//impl From<Arc<AppState>> for TemplateState {
//    fn from(state: Arc<AppState>) -> Self {
//        Self::from(&*state)
//    }
//}

// which page is current navigated to, for navbar formatting
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum CurrentNavigation {
    Upload,
    Paste,
}

#[derive(Template)]
#[template(path = "error.html.jinja")]
pub struct ErrorTemplate<'s> {
    pub state: TemplateState,
    pub code: StatusCode,
    pub message: String,
}

pub enum ErrorResponse {
    Template(ErrorTemplate),
    // currently only used for askama rendering errors. I am not entirely sure when those happen,
    // but if it does, instead of attempting to render ErrorTemplate, it's simpler to just return
    // a bare String to the user so they know something has gone horribly wrong.
    Raw(String),
}
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
