pub mod filters;

use askama::Template;

#[derive(Template)]
#[template(path = "error.html.jinja")]
pub struct ErrorTemplate<'t> {
    pub title: &'t str,
    pub message: &'t str,
}
