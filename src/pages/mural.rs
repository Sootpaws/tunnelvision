use super::templates::template;
use axum::extract::{Path, State};
use axum::response::Response;
use upon::value;
use axum::http::uri::Uri;

pub async fn page(
    State(data): State<crate::data::Data>,
    Path(mural_key): Path<String>,
    uri: Uri,
) -> Response {
    match data.murals.get(&mural_key) {
        Some(mural) => template(
            "mural",
            value! {
                mural: mural,
                mural_key: mural_key,
                tags: mural.lookup_tags(&data),
                artists: mural.lookup_artists(&data)
            },
        ),
        None => super::not_found::page(uri).await,
    }
}
