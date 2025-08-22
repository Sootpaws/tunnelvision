use std::path::Path;
use anyhow::Result;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("{:#}", e),
    }
}

fn run() -> Result<()> {
    let data = tunnelvision::data::load(Path::new("data"))?;
    dbg!(data);
    Ok(())
}
