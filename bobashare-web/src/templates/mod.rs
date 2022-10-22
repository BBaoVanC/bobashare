use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'t> {
    pub title: &'t str,
    pub message: &'t str,
}
