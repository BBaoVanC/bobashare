use std::error::Error;

use askama::Template;
use hyper::StatusCode;

// pub trait TemplateErrorExt: Error + Sized {
//     fn into_error_template_with_code(self, code: StatusCode) -> ErrorTemplate {
//         ErrorTemplate {
//             title: &format!("{} {}", code.as_u16(), code.canonical_reason().unwrap()),
//             message: &self.to_string(),
//         }
//     }
// }

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'t> {
    pub title: &'t str,
    pub message: &'t str,
}
