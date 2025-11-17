use axum::extract::{Path, State};
use axum::http::uri::Uri;
use axum::response::{IntoResponse, Redirect, Response};

pub async fn page(
    State(data): State<crate::data::Data>,
    Path(mural_id): Path<u16>,
    uri: Uri,
) -> Response {
    match data
        .murals
        .iter()
        .find(|(_, mural)| mural.old_id == Some(mural_id))
    {
        Some((key, _)) => Redirect::permanent(&format!("/mural/{key}")).into_response(),
        None => super::not_found::page(uri).await,
    }
}
