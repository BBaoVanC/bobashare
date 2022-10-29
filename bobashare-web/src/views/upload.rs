use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use tracing::instrument;

use super::{ErrorResponse, TemplateState};
use crate::{iter_expiries, AppState};

#[derive(Template)]
#[template(path = "paste.html.jinja")]
pub struct PasteTemplate {
    pub state: TemplateState,
    // value, label, default
    pub expiry_options: Vec<ExpiryOption>,
    pub never_expiry_allowed: bool,
}
#[derive(Debug, Clone)]
pub struct ExpiryOption {
    pub value: &'static str,
    pub label: &'static str,
    pub default: bool,
}

#[instrument(skip(state))]
pub async fn paste(state: State<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    let mut expiry_options = iter_expiries()
        .take_while(|e| {
            if let Some(max) = state.max_expiry {
                e.0 <= max
            } else {
                true
            }
        })
        .map(|(duration, value, label)| ExpiryOption {
            value,
            label,
            // default: duration == state.default_expiry,
            default: false,
        })
        .collect::<Vec<ExpiryOption>>();

    expiry_options.push(ExpiryOption {
        value: format!("{}s", state.default_expiry.num_seconds()),
        default:
    });


    Ok(PasteTemplate {
        expiry_options,
        never_expiry_allowed: state.max_expiry.is_none(),
        state: state.0.into(),
    })
}
