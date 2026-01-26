## Project structure

- `src/main.rs`: Main entry point. Parses command line arguments, calls the data
    loader, and starts the server. All pages must be registered here.
- `src/lib.rs`: Just for organization
- `src/data.rs`: Data loading and image processing, entrypoint is `load`.
    All main data structures are defined here, along with constants for the
    dimensions of display and thumbnail images.
- `src/search.rs`: Defines the search query data structure, and implements the
    filtering and sorting system. This is relatively primitive, and could be
    improved.
- `src/pages/mod.rs`: Just for organization, one module for each page plus the
    `templates` module.
- `src/pages/about.rs`: Renders the about page template
- `src/pages/catalog.rs`: Does a lot of extra work to handle query parameters,
    uses the search system to filter (or not) the murals by search query, and
    fetches and sorts the tag and artist lists for the catalog template.
- `src/pages/error.rs`: Logs an error and renders the error page template. Call
    this on most errors while rendering a page.
- `src/pages/home.rs`: Selects a random mural and renders the home page template
    with it.
- `src/pages/mural.rs`: Fetches the data for a mural and renders the mural
    template
- `src/pages/mural_image.rs`: Hander for mural images. Does some logic to figure
    out where the image is located.
- `src/pages/mural_old.rs`: Redirect for mural URLs from the original version
    of Tunnelvision (`/murals/{id}`). Only works for murals with the `old_id`
    field set.
- `src/pages/not_found.rs`: Fallback for nonexistant pages (both paths like
    `/nonexistant` and `/mural/nonexistant`). Call this on requests for
    nonexistant resources.
- `src/pages/open_canvas.rs`: Open Canvas page. Currenty just fetches a random
    mural for the template.
- `src/pages/statics.rs`: Handles requests for static files. This is both for
    the static data path (`/static/{file}`) and other static files like mural
    images (called via `static_response`). Guesses the content type to send
    based on file extension (there's probably a better way to do this).
- `src/path/templates/mod.rs`: Initializes the templating engine, registers
    templates, and provides templating-related utilities.
- `src/pages/templates/page_pre.html`: Shared template for all pages. Generates
    `meta` tags for embeds, links the CSS, and renders the navbar. This gets
    included at the start of all other pages.
- `src/pages/templates/page_post.html`: Closes the `body` and `html` tags,
    and possibly a footer in the future. Included at the end of every page.
- `src/pages/templates/mural_card.html`: Renders an individual mural card for
    the catalog page.
- `src/pages/templates/*.html`: Main HTML for each page
- `static`: Static files used by the site. Mostly images, plus the shared CSS.
- `Dockerfile`: Deployment-oriented container build instructions
- `Cargo.*`: Rust package manifest
- `flake.*`: Nix flake that packages the `tunnelvision` binary and provides a
    devshell.

## Concepts

### Tag and artist lookup

To make deserializing data easier, tags and artists on murals are represented as
their bare key strings. These are verified at load time so they should always
match an existing tag or artist entry. To get the actual tag and artist objects
associated with a mural, use the `lookup_tags` and `lookup_artists` methods.

### Image processing

To prevent serving extremely large images on every mural page, mural images are
scaled down to a "display" size used on mural pages, as well as a "thumbnail"
size for mural cards on the catalog page. This is done alongside data loading,
subject to caching based on modification time in the specified cache directory.
The `mural_image` module exposes these under the paths
`/mural/{key}/{scale}_{filename}`, where `scale` is one of `display`,
`thumbnail`, or `fullsize`.

## Making additions

### Data fields

- Add a field to the apropriate struct in `src/data.rs`
- Update sample data to include the new field
- Update page templates to display that data
- If the field should contribute to search, add code to `search_mural_all` in
    `src/search.rs` for matching a text representation of the field

### Search terms

- Add a field to the search query struct in `src/search.rs`
- Potentially add code to `Search::normalize` if needed (for things like
    mutually exclusive search options)
- Add a check to `Search::detailed` to make the detailed search menue expand
    when the term is used
- Add checks to `Seach::tag_only`, `Seach::artist_only`, and `Seach::year_only`
- Add code to `Seach::evaluate` to apply the term

### Page

- Create a Rust file for the page's backend logic in `src/pages/{name}.rs`
- Register the backend module in `src/pages/mod.rs`
- Register the page's route in `src/main.rs`
- Create a template file in `src/pages/templates/{name}.html`
- Include the `page_pre` and `page_post` templates to generate the main
    structure of the page
- Register the template in `src/pages/template/mod.rs`
