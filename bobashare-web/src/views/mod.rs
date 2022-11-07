//! Frontend views (as opposed to REST API)

use std::sync::Arc;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use chrono::Duration;
use hyper::StatusCode;
use url::Url;

use crate::AppState;

pub mod api;
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

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::with_state(state)
        .route("/API/", get(api::api))
        .route("/", get(upload::upload))
        .route("/paste/", get(upload::paste))
        .route("/:id", get(display::display))
        .route("/raw/:id", get(display::raw))
}
