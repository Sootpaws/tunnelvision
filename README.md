# Tunnelvision - An interactive catalog of RIT's Murals

This is an in-progress rewrite of the original Tunnelvision, and there are a lot
things that need to be done before this can become the primary version:

- Mural image handling
- Multi-image mural handling
- Meta tags for murals
- Home page
- Catalog page
- Catalog search
- About page
- Open Canvas page

## Development Environment

You will need:

- An installed Rust toolchain ([rustup.rs](https://rustup.rs))
- A mural dataset to display (see the premade test dataset in `sample_data`)

The dataset location is currently hardcoded to `./data`. If using the sample
dataset, `ln -s sample_data data` will create a symbolic link to point the
server to the sample dataset. Use `cargo run` to compile and run the server.

## Dataset Format

All of the information displayed on Tunnelvision is stored in a data
directory, currently fixed as `./data`, in the form of TOML files and images.
The layout of the data directory is as follows:

- `artists.toml`: Information on each artist. An artist must have an entry here
    to be listed as the painter of a mural.
- `tags.toml`: Information on each tag. A tag must have an entry here to be
    listed as a tag on a mural
- `{mural_key}/mural.toml`: Information on the mural with name `mural_key`.
    Images associated with the mural should be put in this directory alongside
    the `mural.toml` file.

### `artists.toml`

Table from artist key string to artist entry. Artist entry:

- `name` - String: Display name of the artist

### `tags.toml`

Table from tag key string to tag entry: Tag entry:

- `name` - String: Display name of the artist

### `{mural_key}/mural.toml`

Mural entry:

- `title` - String: Display name of the mural
- `old_id`- Optional string: Numeric ID for compatability with old Tunnelvision
    mural URLs
- `year` - Positive integer: Year the mural was painted
- `location` - String: Description of the mural's approximate location
- `description` - String: General information on the mural
- `tags` - List of tag keys strings: Tags associated with the mural
- `artist` - List of artist key strings: Artists associated with the mural
- `images` - List of image entries:
    + `filename` - String: File name of the image (relative to `{mural_key}`)
    + `caption` - Optional string: Caption for the image if needed
    + `date` - Date: Date on which the image was taken
    + `by` - String: Image attribution to display
    + `alt` - String: Alt text for the image
