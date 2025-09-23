/// Handler for `/`
pub mod home;

/// Handler for `/mural/{key}`
pub mod mural;

/// Handler for `/mural/{key}/{file}`
pub mod mural_image;

/// Handler for `/murals/{id}` (pre-rewrite style mural IDs)
pub mod mural_old;

/// Handler for `/static/{file}`
pub mod statics;

/// Fallback handler for 404 pages
pub mod not_found;

/// Handler for internal errors
mod error;

/// Templating engine setup
mod templates;
