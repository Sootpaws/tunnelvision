use super::error;
use anyhow::anyhow;
use axum::response::{Html, IntoResponse, Response};
use std::fmt::Write;
use std::sync::LazyLock;
use upon::{Engine, Value, fmt};

/// Global template registry
static TE: LazyLock<Engine<'static>> = LazyLock::new(|| {
    let mut te = Engine::new();

    // Register templates
    te.add_template("page_pre", include_str!("page_pre.html"))
        .unwrap();
    te.add_template("page_post", include_str!("page_post.html"))
        .unwrap();
    te.add_template("mural", include_str!("mural.html"))
        .unwrap();
    te.add_template("not_found", include_str!("not_found.html"))
        .unwrap();
    te.add_template("error", include_str!("error.html"))
        .unwrap();

    te.add_formatter("format_date", format_date);

    te
});

/// Use a template to generate a page
pub fn template(template_name: &str, context: Value) -> Response {
    match TE.get_template(template_name) {
        Some(template) => match template.render_from(&context).to_string() {
            Ok(rendered) => Html(rendered).into_response(),
            Err(error) => error::page(
                <_ as Into<anyhow::Error>>::into(error)
                    .context(format!("While rendering template {template_name}")),
            ),
        },
        None => error::page(anyhow!("Template {template_name} not registered")),
    }
}

/// Cursed date formatting hack
fn format_date(f: &mut fmt::Formatter<'_>, value: &Value) -> fmt::Result {
    match value {
        Value::Map(map) => match map.first_key_value() {
            Some((_, entry)) => match entry {
                Value::String(combined) => {
                    let mut parts = combined.split("-");
                    if let Some(year) = parts.next()
                        && let Some(month) = parts.next()
                        && let Some(day) = parts.next()
                        && let None = parts.next()
                    {
                        write!(f, "{month}/{day}/{year}").map_err(fmt::Error::from)
                    } else {
                        Err(fmt::Error::from("Not date-shaped - incorrect format"))
                    }
                }

                _ => Err(fmt::Error::from("Not date-shaped - entry not string")),
            },
            _ => Err(fmt::Error::from("Not date-shaped - no entries")),
        },
        _ => Err(fmt::Error::from("Not date-shaped - not map")),
    }
}
