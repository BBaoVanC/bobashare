use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use tracing::instrument;

use super::{ErrorResponse, TemplateState};
use crate::AppState;

#[derive(Template)]
#[template(path = "paste.html.jinja")]
pub struct PasteTemplate {
    pub state: TemplateState,
}

#[instrument(skip(state))]
pub async fn paste(state: State<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(PasteTemplate {
        state: state.0.into(),
    })
}
