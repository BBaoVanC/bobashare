use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};

use super::{filters, ErrorResponse, TemplateState};
use crate::AppState;

#[derive(Template)]
#[template(path = "api.html.jinja")]
pub struct ApiTemplate {
    pub state: TemplateState,
}

pub async fn api(state: State<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(ApiTemplate {
        state: state.0.into(),
    })
}
