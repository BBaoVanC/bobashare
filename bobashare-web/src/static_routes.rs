/// Handler to serve static files
use axum::response::IntoResponse;
use hyper::{header, HeaderMap, StatusCode, Uri};
use rust_embed::RustEmbed;
use tracing::{event, instrument, Level};

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

#[instrument(skip(headers), fields(if_none_match = ?headers.get(header::IF_NONE_MATCH)))]
pub async fn handler(uri: Uri, headers: HeaderMap) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    event!(Level::DEBUG, ?path);

    match Asset::get(path) {
        None => {
            event!(Level::WARN, path, "file not found");
            (StatusCode::NOT_FOUND, "404 Not Found").into_response()
        }
        Some(f) => {
            let sha256 = hex::encode(f.metadata.sha256_hash());
            event!(Level::TRACE, sha256);
            if cfg!(not(debug_assertions)) {
                if let Some(tag) = headers.get(header::IF_NONE_MATCH) {
                    event!(Level::TRACE, ?tag);
                    // XXX: cool, a new let-else
                    let Ok(tag) = tag.to_str() else {
                        return (StatusCode::BAD_REQUEST, "invalid ETag").into_response();
                    };
                    if tag == sha256 {
                        return (StatusCode::NOT_MODIFIED, "").into_response();
                    }
                }
            }

            let mimetype = f.metadata.mimetype();
            event!(Level::DEBUG, ?sha256, ?mimetype);
            (
                [
                    (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".to_string()),
                    (header::CACHE_CONTROL, "no-cache".to_string()),
                    (header::CONTENT_TYPE, mimetype.to_string()),
                    (header::ETAG, sha256),
                ],
                f.data,
            )
                .into_response()
        }
    }
}
