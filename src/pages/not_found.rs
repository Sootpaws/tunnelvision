use super::templates::template;
use axum::http::uri::Uri;
use axum::response::Response;
use upon::value;

pub async fn page(uri: Uri) -> Response {
    eprintln!("Page not found: {uri}");
    template("not_found", value! {})
}
