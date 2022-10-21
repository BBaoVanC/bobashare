use askama::Template;
use axum::response::IntoResponse;
use hyper::header;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'t> {
    pub title: &'t str,
    pub message: &'t str,
}
impl IntoResponse for ErrorTemplate<'_> {
    fn into_response(self) -> axum::response::Response {
        (
            [(header::CONTENT_TYPE, Self::MIME_TYPE)],
            self.render().unwrap(),
        ).into_response()
    }
}
