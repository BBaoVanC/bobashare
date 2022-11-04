/// Handler to serve static files
use axum::{headers::IfNoneMatch, response::IntoResponse, TypedHeader};
use hyper::{header, StatusCode, Uri};
use rust_embed::RustEmbed;
use tracing::{event, instrument, Level};

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

#[instrument]
pub async fn handler(
    uri: Uri,
    if_none_match: Option<TypedHeader<IfNoneMatch>>,
) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    event!(Level::DEBUG, ?path);

    match Asset::get(path) {
        None => {
            event!(Level::WARN, path, "file not found");
            (StatusCode::NOT_FOUND, "404 Not Found").into_response()
        }
        Some(f) => {
            // TODO: logging
            let sha256 = hex::encode(f.metadata.sha256_hash());
            // TODO: verify that this condition works properly in release
            if cfg!(not(debug_assertions)) {
                if let Some(tag) = if_none_match {
                    let etag = format!("\"{}\"", sha256);
                    if tag.0.precondition_passes(&etag.parse().unwrap()) {
                        return (StatusCode::NOT_MODIFIED, "").into_response();
                    }
                }
            }

            let mimetype = mime_db::lookup(path)
                .map_or(mime::APPLICATION_OCTET_STREAM, |m| m.parse().unwrap());
            event!(Level::DEBUG, ?sha256, ?mimetype);
            (
                [
                    (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".to_string()),
                    (header::CACHE_CONTROL, "no-cache".to_string()),
                    (header::CONTENT_TYPE, mimetype.to_string()),
                    (header::ETAG, hex::encode(f.metadata.sha256_hash())),
                ],
                f.data,
            )
                .into_response()
        }
    }
}
