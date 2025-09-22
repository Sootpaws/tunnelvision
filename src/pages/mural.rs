use super::templates::template;
use axum::extract::{Path, State};
use axum::response::Response;
use upon::value;

pub async fn page(
    State(data): State<crate::data::Data>,
    Path(mural_key): Path<String>,
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
        None => super::not_found::page().await,
    }
}
