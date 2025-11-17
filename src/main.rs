use anyhow::Result;
use axum::{Router, routing::get};
use clap::Parser;
use std::path::PathBuf;
use tunnelvision::pages;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => (),
        Err(e) => eprintln!("{e:#}"),
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();

    let data = tunnelvision::data::load(&args.data_path, &args.images_path)?;

    let app = Router::new()
        .route("/", get(pages::home::page))
        .route("/catalog", get(pages::catalog::page))
        .route("/mural/{key}", get(pages::mural::page))
        .route("/mural/{key}/{file}", get(pages::mural_image::page))
        .route("/murals/{id}", get(pages::mural_old::page))
        .route("/open-canvas", get(pages::open_canvas::page))
        .route("/about", get(pages::about::page))
        .route("/static/{file}", get(pages::statics::page))
        .fallback(pages::not_found::page)
        .with_state(data);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path from which to load mural data
    #[arg(short, long)]
    data_path: PathBuf,
    /// Path to use for caching resized images
    #[arg(short, long)]
    images_path: PathBuf,
}
