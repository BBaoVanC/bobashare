/// Handler to serve static files
use axum::response::IntoResponse;
use hyper::{header, StatusCode, Uri};
use rust_embed::RustEmbed;
use tracing::{event, instrument, Level};

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

#[instrument]
pub async fn handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    event!(Level::DEBUG, ?path);

    match Asset::get(path) {
        None => {
            event!(Level::WARN, "file not found");
            (StatusCode::NOT_FOUND, "404 Not Found").into_response()
        }
        Some(f) => {
            let mimetype = mime_db::lookup(&path)
                .map_or(mime::APPLICATION_OCTET_STREAM, |m| m.parse().unwrap());
            event!(Level::DEBUG, ?mimetype);
            ([(header::CONTENT_TYPE, mimetype.to_string())], f.data).into_response()
        }
    }
}
