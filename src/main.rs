use std::path::Path;
use anyhow::Result;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => eprintln!("{:#}", e),
    }
}

async fn run() -> Result<()> {
    let data = tunnelvision::data::load(Path::new("data"))?;
    dbg!(data);
    Ok(())
}
