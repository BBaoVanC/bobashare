use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};

use super::{ErrorResponse, TemplateState};
use crate::AppState;
use super::filters;

#[derive(Template)]
#[template(path = "about.html.jinja")]
pub struct AboutTemplate {
    pub state: TemplateState,
}

pub async fn about(state: State<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(AboutTemplate {
        state: state.0.into(),
    })
}
