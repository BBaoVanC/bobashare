use askama::Template;
use axum::{
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use hyper::header;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'t> {
    pub title: &'t str,
    pub message: &'t str,
}
impl IntoResponse for ErrorTemplate<'_> {
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(Self::MIME_TYPE),
            )],
            self.render().unwrap(),
        )
            .into_response()
    }
}
