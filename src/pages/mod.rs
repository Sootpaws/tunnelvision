/// Handler for `/mural/{name}`
pub mod mural;

/// Handler for `/static/{file}`
pub mod statics;

/// Fallback handler for 404 pages
pub mod not_found;

/// Handler for internal errors
mod error;

/// Templating engine setup
mod templates;
