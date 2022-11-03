use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use chrono::Duration;
use tracing::{event, instrument, Level};

use super::{filters, ErrorResponse, TemplateState};
use crate::{ AppState};

#[derive(Debug, Clone)]
pub struct ExpiryUnit<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub default: bool,
    pub duration: Duration,
}
pub fn iter_expiry_units() -> impl Iterator<Item = ExpiryUnit<'static>> {
    [
        ExpiryUnit {
            name: "seconds",
            value: "s",
            default: false,
            duration: Duration::seconds(1),
        },
        ExpiryUnit {
            name: "minutes",
            value: "m",
            default: false,
            duration: Duration::minutes(1),
        },
        ExpiryUnit {
            name: "hours",
            value: "h",
            default: false,
            duration: Duration::hours(1),
        },
        ExpiryUnit {
            name: "days",
            value: "d",
            default: true,
            duration: Duration::days(1),
        },
        ExpiryUnit {
            name: "weeks",
            value: "w",
            default: false,
            duration: Duration::days(7),
        },
        ExpiryUnit {
            name: "months",
            value: "mon",
            default: false,
            duration: Duration::days(30),
        },
        ExpiryUnit {
            name: "years",
            value: "y",
            default: false,
            duration: Duration::days(365)
        },
    ]
    .into_iter()
}

#[derive(Template)]
#[template(path = "upload.html.jinja")]
pub struct UploadTemplate<'a> {
    pub state: TemplateState,
    // TODO: make this iterator and not vec
    pub expiry_units: Vec<ExpiryUnit<'a>>,
    pub never_expiry_allowed: bool,
}

#[instrument(skip(state))]
pub async fn upload(state: State<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    event!(Level::DEBUG, "returning upload template");
    Ok(UploadTemplate {
        expiry_units: iter_expiry_units().take_while(|e| {
            if let Some(max) = state.max_expiry {
                max >= e.duration
            } else {
                true
            }
        }).collect(),
        // TODO: make never expiry work
        never_expiry_allowed: state.max_expiry.is_none(),
        state: state.0.into(),
    })
}

#[derive(Template)]
#[template(path = "paste.html.jinja")]
pub struct PasteTemplate<'a> {
    pub state: TemplateState,
    pub expiry_units: Vec<ExpiryUnit<'a>>,
    pub never_expiry_allowed: bool,
}

#[instrument(skip(state))]
pub async fn paste(state: State<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    event!(Level::DEBUG, "returning paste template");
    Ok(PasteTemplate {
        expiry_units: iter_expiry_units().take_while(|e| {
            if let Some(max) = state.max_expiry {
                max >= e.duration
            } else {
                true
            }
        }).collect(),
        never_expiry_allowed: state.max_expiry.is_none(),
        state: state.0.into(),
    })
}
