use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};
use chrono::TimeDelta;
use tracing::{event, instrument, Level};

use super::{filters, CurrentNavigation, ErrorResponse, TemplateState};
use crate::AppState;

#[derive(Debug, Clone)]
pub struct ExpiryUnit<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub default: bool,
    pub duration: TimeDelta,
}
pub fn iter_expiry_units() -> impl Iterator<Item = ExpiryUnit<'static>> {
    [
        ExpiryUnit {
            name: "seconds",
            value: "s",
            default: false,
            duration: TimeDelta::try_seconds(1).unwrap(),
        },
        ExpiryUnit {
            name: "minutes",
            value: "m",
            default: false,
            duration: TimeDelta::try_minutes(1).unwrap(),
        },
        ExpiryUnit {
            name: "hours",
            value: "h",
            default: false,
            duration: TimeDelta::try_hours(1).unwrap(),
        },
        ExpiryUnit {
            name: "days",
            value: "d",
            default: true,
            duration: TimeDelta::try_days(1).unwrap(),
        },
        ExpiryUnit {
            name: "weeks",
            value: "w",
            default: false,
            duration: TimeDelta::try_days(7).unwrap(),
        },
        ExpiryUnit {
            name: "months",
            value: "mon",
            default: false,
            duration: TimeDelta::try_days(30).unwrap(),
        },
        ExpiryUnit {
            name: "years",
            value: "y",
            default: false,
            duration: TimeDelta::try_days(365).unwrap(),
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
    let mut state: TemplateState = state.0.into();
    state.current_navigation = Some(CurrentNavigation::Upload);
    event!(Level::DEBUG, "returning upload template");
    Ok(UploadTemplate {
        expiry_units: iter_expiry_units()
            .take_while(|e| {
                if let Some(max) = state.max_expiry {
                    max >= e.duration
                } else {
                    true
                }
            })
            .collect(),
        never_expiry_allowed: state.max_expiry.is_none(),
        state,
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
    let mut state: TemplateState = state.0.into();
    state.current_navigation = Some(CurrentNavigation::Paste);
    event!(Level::DEBUG, "returning paste template");
    Ok(PasteTemplate {
        expiry_units: iter_expiry_units()
            .take_while(|e| {
                if let Some(max) = state.max_expiry {
                    max >= e.duration
                } else {
                    true
                }
            })
            .collect(),
        never_expiry_allowed: state.max_expiry.is_none(),
        state,
    })
}
