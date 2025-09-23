use anyhow::Result;
use axum::{Router, routing::get};
use std::path::Path;
use tunnelvision::pages;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => eprintln!("{e:#}"),
    }
}

async fn run() -> Result<()> {
    let data = tunnelvision::data::load(Path::new("data"))?;

    let app = Router::new()
        .route("/", get(pages::home::page))
        .route("/mural/{key}", get(pages::mural::page))
        .route("/mural/{key}/{file}", get(pages::mural_image::page))
        .route("/murals/{id}", get(pages::mural_old::page))
        .route("/static/{file}", get(pages::statics::page))
        .fallback(pages::not_found::page)
        .with_state(data);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
