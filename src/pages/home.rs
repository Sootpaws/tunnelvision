use super::templates::template;
use axum::extract::{Path, State};
use axum::response::Response;
use upon::value;

pub async fn page(State(data): State<crate::data::Data>) -> Response {
    template("home", value! {})
}
