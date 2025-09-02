use crate::pages::not_found;
use axum::extract;
use axum::http::header;
use axum::response::{IntoResponse, Response};
use std::{fs, path};

const STATIC_PATH: &str = "src/pages/static";

pub async fn page(extract::Path(file): extract::Path<String>) -> Response {
    let file_path = path::Path::new(STATIC_PATH).join(&file);
    match file_path.try_exists() {
        Ok(true) => match fs::read(file_path) {
            Ok(contents) => {
                ([(header::CONTENT_TYPE, content_type(&file))], contents).into_response()
            }
            Err(error) => format!("sad 2 {error:?}").into_response(),
        },
        Ok(false) => not_found::page().await,
        Err(error) => format!("sad {error:?}").into_response(),
    }
}

/// Determine the content type to specify for a given file
fn content_type(file: &str) -> &str {
    if file.ends_with(".css") {
        "text/css"
    } else {
        "text/plain"
    }
}
