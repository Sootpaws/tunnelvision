use crate::pages::statics::static_response;
use axum::extract::{Path, State};
use axum::http::uri::Uri;
use axum::response::Response;

pub async fn page(
    State(data): State<crate::data::Data>,
    Path((mural_key, file)): Path<(String, String)>,
    uri: Uri,
) -> Response {
    let path = if let Some(original) = file.strip_prefix("fullsize_") {
        data.source.join(&mural_key).join(original)
    } else {
        data.image_store.join(&mural_key).join(&file)
    };
    static_response(&path, file, uri).await
}
