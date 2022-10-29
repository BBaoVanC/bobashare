use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use chrono::Duration;
use tracing::instrument;

use super::{filters, ErrorResponse, TemplateState};
use crate::{iter_default_expiries, AppState};

#[derive(Template)]
#[template(path = "paste.html.jinja")]
pub struct PasteTemplate {
    pub state: TemplateState,
    // duration, default
    pub expiry_options: Vec<(Duration, bool)>,
    pub never_expiry_allowed: bool,
}

#[instrument(skip(state))]
pub async fn paste(state: State<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    // TODO: this is horrific
    let mut expiry_options = iter_default_expiries()
        .take_while(|e| {
            if let Some(max) = state.max_expiry {
                e <= &max
            } else {
                true
            }
        })
        .collect::<Vec<Duration>>();
    expiry_options.push(state.default_expiry);
    expiry_options.sort();
    expiry_options.dedup();

    let expiry_options = expiry_options
        .into_iter()
        .map(|e| (e, e == state.default_expiry))
        .collect();

    Ok(PasteTemplate {
        expiry_options,
        never_expiry_allowed: state.max_expiry.is_none(),
        state: state.0.into(),
    })
}
