//! Frontend views (as opposed to REST API)

use std::sync::Arc;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use hyper::StatusCode;
use url::Url;

use crate::AppState;

pub mod filters;
pub mod display;

#[derive(Debug, Clone)]
pub struct TemplateState {
    base_url: Url,
}
impl From<&AppState> for TemplateState {
    fn from(state: &AppState) -> Self {
        Self {
            base_url: state.base_url.clone(),
        }
    }
}
impl From<Arc<AppState>> for TemplateState {
    fn from(state: Arc<AppState>) -> Self {
        Self {
            base_url: state.base_url.clone(),
        }
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
        .route("/:id", get(display::display))
        .route("/raw/:id", get(display::raw))
}
