use serde::Deserialize;
use toml::value::Date;
use std::collections::HashMap;
use anyhow::{anyhow, bail, Context, Result};
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct Data {
    artists: HashMap<String, Artist>,
    tags: HashMap<String, Tag>,
    murals: HashMap<String, Mural>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Mural {
    title: String,
    year: u16,
    location: String,
    description: String,
    tags: Vec<String>,
    artists: Vec<String>,
    images: Vec<Image>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Image {
    filename: String,
    caption: Option<String>,
    date: Date,
    by: String,
    alt: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Artist {
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    name: String,
}

pub fn load(from: &Path) -> Result<Data> {
    // Load artist data
    let artists_path = from.join(Path::new("artists.toml"));
    let artists_file = fs::read_to_string(artists_path)
        .context("Could not read artists file")?;
    let artists: HashMap<String, Artist> = toml::from_str(&artists_file)
        .context("Could not parse artists file")?;
    // Load tag data
    let tags_path = from.join(Path::new("tags.toml"));
    let tags_file = fs::read_to_string(tags_path)
        .context("Could not read tags file")?;
    let tags: HashMap<String, Tag> = toml::from_str(&tags_file)
        .context("Could not parse tags file")?;
    // Load murals
    let mut murals: HashMap<String, Mural> = HashMap::new();
    for entry in std::fs::read_dir(from)
            .context("Could not scan data directory")? {
        let entry = entry
            .context("Could not read data entry")?;
        let path = entry.path();
        if !path.is_dir() { continue }
        // Load mural data
        let mural_path = path.join("mural.toml");
        let mural_file = fs::read_to_string(mural_path)
            .context("Could not read mural file")?;
        let mural: Mural = toml::from_str(&mural_file)
            .context("Could not path mural file")?;
        // Verify artists and tags
        for artist in &mural.artists {
            if !artists.contains_key(artist) {
                bail!("Mural has artist {artist} which is not listed in artists file");
            }
        }
        for tag in &mural.tags {
            if !tags.contains_key(tag) {
                bail!("Mural has tag {tag} which is not listed in tags file");
            }
        }
        // Add to mural list
        murals.insert(
            entry.file_name().into_string()
                .map_err(|bad| anyhow!(
                    "Mural directory name is invalid: {}",
                    bad.display()
                ))?,
            mural,
        );
    }
    Ok(Data { artists, tags, murals })
}
