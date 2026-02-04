use anyhow::{Context, Result, anyhow, bail, ensure};
use image::ImageReader;
use image::imageops::FilterType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml::value::Date;

#[derive(Clone, Debug)]
pub struct Data {
    pub artists: HashMap<String, Artist>,
    pub tags: HashMap<String, Tag>,
    pub murals: HashMap<String, Mural>,
    pub source: PathBuf,
    pub image_store: PathBuf,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Mural {
    pub title: String,
    pub old_id: Option<u16>,
    pub year: u16,
    pub location: String,
    pub description: String,
    pub tags: Vec<String>,
    pub artists: Vec<String>,
    pub images: Vec<Image>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Image {
    pub filename: String,
    pub caption: Option<String>,
    pub date: Date,
    pub by: String,
    #[serde(default)]
    pub alt: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Artist {
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Tag {
    pub name: String,
    pub description: String,
}

pub fn load(source: &Path, image_store: &Path) -> Result<Data> {
    println!("Loading/processing data...");
    // Load artist data
    let artists_path = source.join(Path::new("artists.toml"));
    let artists_file = fs::read_to_string(artists_path).context("Could not read artists file")?;
    let artists: HashMap<String, Artist> =
        toml::from_str(&artists_file).context("Could not parse artists file")?;
    // Load tag data
    let tags_path = source.join(Path::new("tags.toml"));
    let tags_file = fs::read_to_string(tags_path).context("Could not read tags file")?;
    let tags: HashMap<String, Tag> =
        toml::from_str(&tags_file).context("Could not parse tags file")?;
    // Load murals
    let mut murals = HashMap::new();
    load_murals(source, image_store, &artists, &tags, &mut murals)?;
    println!("Done");
    Ok(Data {
        artists,
        tags,
        murals,
        source: source.to_owned(),
        image_store: image_store.to_owned(),
    })
}

/// Recursively load murals from a directory
fn load_murals(
    source: &Path,
    image_store: &Path,
    artists: &HashMap<String, Artist>,
    tags: &HashMap<String, Tag>,
    murals: &mut HashMap<String, Mural>,
) -> Result<()> {
    for entry in std::fs::read_dir(source).context("Could not scan data directory")? {
        let entry = entry.context("Could not read data entry")?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let mural_path = path.join("mural.toml");
        if fs::exists(&mural_path).context("Could not check if mural.toml exists")? {
            let mural_key = entry
                .file_name()
                .into_string()
                .map_err(|bad| anyhow!("Mural directory name is invalid: {}", bad.display()))?;
            // Load mural data
            let mural_file = fs::read_to_string(mural_path).context("Could not read mural file")?;
            let mural: Mural = toml::from_str(&mural_file).context("Could not parse mural file")?;
            // Check that there is at least one image
            if mural.images.is_empty() {
                bail!("Mural has no images, at least one is required");
            }
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
            // Process images
            mural
                .process_images(&path, &image_store.join(&mural_key), &mural_key)
                .context(format!("Could not process images for mural {mural_key}"))?;
            // Add to mural list
            ensure!(
                murals.insert(mural_key.clone(), mural).is_none(),
                "Duplicate mural key {mural_key}"
            );
        } else {
            load_murals(&path, image_store, artists, tags, murals)?;
        }
    }
    Ok(())
}

const DISPLAY_IMAGE_WIDTH: u32 = 1600;
const DISPLAY_IMAGE_HEIGHT: u32 = 1200;
const THUMBNAIL_IMAGE_WIDTH: u32 = 256;
const THUMBNAIL_IMAGE_HEIGHT: u32 = 256;

impl Mural {
    /// Perform a lookup of this mural's tag keys
    pub fn lookup_tags<'a>(&self, data: &'a Data) -> Vec<(&str, &'a Tag)> {
        self.tags
            .iter()
            .map(|key| (key.as_str(), data.tags.get(key).unwrap()))
            .collect()
    }

    /// Perform a lookup of this mural's artist keys
    pub fn lookup_artists<'a>(&self, data: &'a Data) -> Vec<(&str, &'a Artist)> {
        self.artists
            .iter()
            .map(|key| (key.as_str(), data.artists.get(key).unwrap()))
            .collect()
    }

    /// Generate display and thumbnail versions of associated images
    pub fn process_images(&self, from: &Path, to: &Path, mural_key: &str) -> Result<()> {
        fs::create_dir_all(to).context("Could not create image store directory")?;
        for (index, image) in self.images.iter().enumerate() {
            let generate_thumbnail = index == 0;
            // Generate paths
            let source_path = from.join(&image.filename);
            let fullsize_path = to.join(format!("fullsize_{}", image.filename));
            let display_path = to.join(format!("display_{}", image.filename));
            let thumbnail_path = to.join(format!("thumbnail_{}", image.filename));
            // Check resize freshness
            let source_modified = fs::metadata(&source_path)
                .context("Could not get source file metadata")?
                .modified()
                .context("Could not get source file modification time")?;
            let fullsize_fresh =
                if fs::exists(&fullsize_path).context("Could not check if fullsize file exists")? {
                    let fullsize_modified = fs::metadata(&fullsize_path)
                        .context("Could not get fullsize file metadata")?
                        .modified()
                        .context("Could not get fullsize file modification time")?;
                    fullsize_modified > source_modified
                } else {
                    false
                };
            let display_fresh =
                if fs::exists(&display_path).context("Could not check if display file exists")? {
                    let display_modified = fs::metadata(&display_path)
                        .context("Could not get display file metadata")?
                        .modified()
                        .context("Could not get diplay file modification time")?;
                    display_modified > source_modified
                } else {
                    false
                };
            let thumbnail_fresh = if fs::exists(&thumbnail_path)
                .context("Could not check if thumbnail file exists")?
            {
                let thumbnail_modified = fs::metadata(&thumbnail_path)
                    .context("Could not get thumbnail file metadata")?
                    .modified()
                    .context("Could not get thumbnail file modification time")?;
                thumbnail_modified > source_modified
            } else {
                !generate_thumbnail
            };
            // Skip processing if processed versions alreay exist and are frehs
            if fullsize_fresh && display_fresh && thumbnail_fresh {
                continue;
            }
            println!("Processing {mural_key}/{}", image.filename);
            // Link fullsize image
            let full = ImageReader::open(source_path)
                .context(format!("Could not open source image {}", image.filename))?
                .decode()
                .context(format!("Could not decode source image {}", image.filename))?;
            // TODO: This could be a symlink
            full.save(fullsize_path).context("Could not save fullsize image")?;
            // Process display size image
            let display = full.resize(
                DISPLAY_IMAGE_WIDTH,
                DISPLAY_IMAGE_HEIGHT,
                FilterType::CatmullRom,
            );
            display
                .save(display_path)
                .context("Could not save display image")?;
            // Process thumbnail
            if generate_thumbnail {
                display
                    .thumbnail(THUMBNAIL_IMAGE_WIDTH, THUMBNAIL_IMAGE_HEIGHT)
                    .save(thumbnail_path)
                    .context("Could not save thumbnail image")?;
            }
        }
        Ok(())
    }
}
