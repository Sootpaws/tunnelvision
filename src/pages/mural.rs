use axum::extract::{Path, State};

pub async fn page(
    State(data): State<crate::data::Data>,
    Path(mural_key): Path<String>,
) -> String {
    format!("Mural {mural_key}")
}
