use super::templates::template;
use axum::response::Response;
use upon::value;
use axum::http::uri::Uri;

pub async fn page(uri: Uri) -> Response {
    eprintln!("Page not found: {uri}");
    template("not_found", value! {})
}
