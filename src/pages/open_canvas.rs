use super::templates::template;
use axum::extract::State;
use axum::response::Response;
use upon::value;

pub async fn page(State(data): State<crate::data::Data>) -> Response {
    let (highlight_key, highlight_mural) = fastrand::choice(data.murals.iter()).unwrap();
    let highlight_image = &fastrand::choice(highlight_mural.images.iter()).unwrap();
    template(
        "open_canvas",
        value! {
            highlight_key: highlight_key,
            highlight_image: highlight_image
        },
    )
}
