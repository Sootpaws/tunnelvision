use crate::pages::statics::static_response;
use axum::extract;
use axum::response::Response;
use std::path;

const STATIC_PATH: &str = "data";

pub async fn page(extract::Path((mural_key, file)): extract::Path<(String, String)>) -> Response {
    static_response(
        &path::Path::new(STATIC_PATH).join(&mural_key).join(&file),
        file,
    )
    .await
}
