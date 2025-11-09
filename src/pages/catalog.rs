use super::templates::template;
use crate::search::Search;
use axum::extract::{State, Query};
use axum::response::Response;
use upon::value;

pub async fn page(
    State(data): State<crate::data::Data>,
    query: Query<Search>,
) -> Response {
    let murals = query.apply(&data.murals);
    template("catalog", value! { murals: murals })
}
