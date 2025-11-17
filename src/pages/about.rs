use super::templates::template;
use axum::response::Response;
use upon::value;

pub async fn page() -> Response {
    template("about", value! {})
}
