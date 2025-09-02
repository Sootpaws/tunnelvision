use axum::response::Response;
use super::templates::template;
use upon::value;

pub async fn page() -> Response {
    template("not_found", value! {})
}
