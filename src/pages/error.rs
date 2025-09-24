use super::templates::template;
use anyhow::Error;
use axum::response::Response;
use upon::value;

pub fn page(error: Error) -> Response {
    eprintln!("{error:#}");
    template("error", value! {})
}
