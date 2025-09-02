use axum::response::Response;
use anyhow::Error;
use super::templates::template;
use upon::value;

pub fn page(error: Error) -> Response {
    eprintln!("{error:#}");
    template("error", value!{})
}
