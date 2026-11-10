use crate::pages::statics::static_response;
use axum::extract::{Path, State};
use axum::http::uri::Uri;
use axum::response::Response;

pub async fn page(
    State(data): State<crate::data::Data>,
    Path((mural_key, file)): Path<(String, String)>,
    uri: Uri,
) -> Response {
    static_response(&data.image_store.join(&mural_key).join(&file), file, uri).await
}
