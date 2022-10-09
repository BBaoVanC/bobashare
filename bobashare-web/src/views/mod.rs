use std::sync::Arc;

use axum::{extract::{State, Path}, response::IntoResponse};

use crate::AppState;

use axum::response::Result;

pub async fn display(
    state: State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    Ok(format!("Hello {}", state.base_url.join(&id).unwrap()))
}
