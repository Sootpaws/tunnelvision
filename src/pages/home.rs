use super::templates::template;
use axum::extract::State;
use axum::response::Response;
use upon::value;

pub async fn page(State(data): State<crate::data::Data>) -> Response {
    let (bg_key, bg_mural) = fastrand::choice(data.murals.iter()).unwrap();
    let bg_file = &fastrand::choice(bg_mural.images.iter()).unwrap().filename;
    template("home", value! { bg_key: bg_key, bg_file: bg_file })
}
