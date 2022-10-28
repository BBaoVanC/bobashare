use std::sync::Arc;

use askama::Template;
use axum::{response::IntoResponse, extract::State};
use tracing::instrument;

use crate::AppState;

use super::{ErrorResponse, TemplateState};

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
