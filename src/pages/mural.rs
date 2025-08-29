use axum::extract::Path;

pub async fn page(Path(mural_key): Path<String>) -> String {
    format!("Mural {mural_key}")
}
