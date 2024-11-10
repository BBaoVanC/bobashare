use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use hyper::StatusCode;
use pulldown_cmark::{html::push_html, Parser};
use tracing::instrument;

use super::{
    filters, render_template, CurrentNavigation, ErrorResponse, ErrorTemplate, TemplateState,
};
use crate::{AppState, MARKDOWN_OPTIONS};

#[derive(Template)]
#[template(path = "about.html.jinja")]
pub struct AboutTemplate<'s> {
    pub state: TemplateState<'s>,
    pub about_content_rendered: String,
}

/// Display a simple about page
#[instrument(skip(state))]
pub async fn about(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    let mut tmpl_state = TemplateState::from(&*state);

    if state.about_page.is_none() {
        Err(ErrorTemplate {
            state: tmpl_state.clone(),
            code: StatusCode::NOT_FOUND,
            message: "no about page is configured".to_string(),
        })?;
    }

    tmpl_state.current_navigation = Some(CurrentNavigation::About);

    let mut parser = Parser::new_ext(&state.about_page_content, MARKDOWN_OPTIONS);
    let mut about_content_rendered = String::new();
    push_html(&mut about_content_rendered, parser);

    render_template(AboutTemplate {
        state: tmpl_state,
        about_content_rendered,
    })
}
