pub mod filters;

use askama::Template;

use crate::AppState;

#[derive(Template)]
#[template(path = "error.html.jinja")]
pub struct ErrorTemplate<'t> {
    pub state: &'t AppState,
    pub title: &'t str,
    pub message: &'t str,
}
