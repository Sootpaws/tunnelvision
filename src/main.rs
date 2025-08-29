use std::path::Path;
use std::sync::Arc;
use anyhow::Result;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => eprintln!("{:#}", e),
    }
}

async fn run() -> Result<()> {
    let data = tunnelvision::data::load(Path::new("data"))?;

    let app = Router::new()
        .route("/", get(|| async { "Tunnelvision" }))
        .route("/mural/{key}", get(tunnelvision::pages::mural::page))
        .with_state(data);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
