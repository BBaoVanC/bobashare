use askama::Template;
use axum::{extract::State, response::IntoResponse};
use hyper::StatusCode;
use tracing::instrument;

use super::{
    filters, render_template, CurrentNavigation, ErrorResponse, ErrorTemplate, TemplateState,
};
use crate::AppState;

#[derive(Template)]
#[template(path = "about.html.jinja")]
pub struct AboutTemplate<'s, 'c> {
    pub state: TemplateState<'s>,
    pub about_content_rendered: &'c str,
}

/// Display a simple about page
#[instrument(skip(state))]
pub async fn about(
    State(state): State<&'static AppState>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut tmpl_state = TemplateState::from(state);

    if state.about_page.is_none() {
        Err(ErrorTemplate {
            state: tmpl_state.clone(),
            code: StatusCode::NOT_FOUND,
            message: "no about page is configured".to_string(),
        })?;
    }

    tmpl_state.current_navigation = Some(CurrentNavigation::About);

    render_template(AboutTemplate {
        state: tmpl_state,
        about_content_rendered: &state.about_page_content,
    })
}
