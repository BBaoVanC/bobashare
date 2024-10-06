//! Frontend views (as opposed to REST API)

use std::sync::Arc;

use askama::Template;
use axum::{
    http,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
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
// TODO: should this be Copy
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

// which page is current navigated to, for navbar formatting
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum CurrentNavigation {
    Upload,
    Paste,
}

#[derive(Template)]
#[template(path = "error.html.jinja")]
pub struct ErrorTemplate<'s> {
    pub state: TemplateState<'s>,
    pub code: StatusCode,
    pub message: String,
}

pub struct ErrorResponse(Response);
impl From<ErrorTemplate<'_>> for ErrorResponse {
    fn from(tmpl: ErrorTemplate) -> Self {
        let error_msg = &tmpl.message;
        match tmpl.render() {
            Ok(s) => {
                let status_num = tmpl.code.as_u16();
                if tmpl.code.is_server_error() {
                    event!(Level::ERROR, status = status_num, error_msg);
                } else if tmpl.code.is_client_error() {
                    event!(Level::WARN, status = status_num, error_msg);
                } else {
                    event!(Level::INFO, status = status_num, error_msg);
                }
                Self(
                    (
                        tmpl.code,
                        [(
                            http::header::CONTENT_TYPE,
                            http::header::HeaderValue::from_static(ErrorTemplate::MIME_TYPE),
                        )],
                        s,
                    )
                        .into_response(),
                )
            }
            Err(e) => {
                let status = tmpl.code.as_u16();
                event!(Level::ERROR, status, error_msg, render_error = ?e, "error rendering error page template, so HTTP 500 returned:");
                Self(
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("internal error rendering error page template: {:?}", e),
                    )
                        .into_response(),
                )
            }
        }
    }
}
impl From<askama::Error> for ErrorResponse {
    fn from(err: askama::Error) -> Self {
        event!(Level::ERROR, render_error = ?err, "error rendering template");
        Self(
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal error rendering template: {:?}", err),
            )
                .into_response(),
        )
    }
}
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}

pub(crate) fn render_template<T: askama::Template>(tmpl: T) -> Result<Response, ErrorResponse> {
    let rendered = tmpl.render()?;
    Ok((
        StatusCode::OK,
        [(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static(T::MIME_TYPE),
        )],
        rendered,
    )
        .into_response())
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(upload::upload))
        .route("/paste/", get(upload::paste))
        .route("/:id", get(display::display))
        .route("/raw/:id", get(display::raw))
}
