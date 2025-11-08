use super::templates::template;
use axum::extract::State;
use axum::response::Response;
use upon::value;

pub async fn page(State(data): State<crate::data::Data>) -> Response {
    let murals = data.murals.values().collect::<Vec<_>>();
    template("catalog", value! { murals: murals })
}
