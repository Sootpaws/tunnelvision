use crate::data::Mural;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct Search {
    // Generic search over text and metadata
    query: String,
    // Filter by associated text
    text: String,
    // Filter by year painted
    min_year: Option<u16>,
    max_year: Option<u16>,
    year: Option<u16>,
    by_decade: bool,
}

impl Search {
    /// Filter and order a set of murals
    pub fn apply<'a>(&self, murals: &'a HashMap<String, Mural>) -> Vec<(&'a String, &'a Mural)> {
        murals.iter().filter_map(|m| self.evaluate(m)).collect()
    }

    /// Normalize search terms after deserialization
    pub fn normalize(&mut self) {
        if self.year.is_some() {
            self.min_year = None;
            self.max_year = None;
        }
    }

    /// Check if the search uses detailed filtering
    pub fn detailed(&self) -> bool {
        !self.text.is_empty()
            || self.min_year.is_some()
            || self.max_year.is_some()
            || self.year.is_some()
            || self.by_decade
    }

    /// Check if the search filters by year only
    pub fn year_only(&self) -> bool {
        self.year.is_some()
            && self.query.is_empty()
            && self.text.is_empty()
            && self.min_year.is_none()
            && self.max_year.is_none()
    }

    fn evaluate<'a>(&self, mural: (&'a String, &'a Mural)) -> Option<(&'a String, &'a Mural)> {
        // Filter by general query
        if !self.query.is_empty()
            && !self
                .query
                .split(' ')
                .all(|term| search_mural_all(mural.1, term))
        {
            return None;
        }
        // Filter by text
        if !self.text.is_empty()
            && !self
                .text
                .split(' ')
                .all(|term| search_mural_text(mural.1, term))
        {
            return None;
        }

        // Filter by year
        let mural_year = if self.by_decade {
            mural.1.year / 10
        } else {
            mural.1.year
        };
        let min_year = self
            .min_year
            .map(|v| if self.by_decade { v / 10 } else { v });
        let max_year = self
            .max_year
            .map(|v| if self.by_decade { v / 10 } else { v });
        let year = self.year.map(|v| if self.by_decade { v / 10 } else { v });

        if let Some(min_year) = min_year
            && mural_year < min_year
        {
            return None;
        }
        if let Some(max_year) = max_year
            && mural_year > max_year
        {
            return None;
        }
        if let Some(year) = year
            && mural_year != year
        {
            return None;
        }
        Some(mural)
    }
}

/// Search the main text content of a mural for a specific term
fn search_mural_text(mural: &Mural, term: &str) -> bool {
    mural.title.contains(term)
        || mural.description.contains(term)
        || mural.images.iter().any(|image| {
            image.filename.contains(term)
                || image
                    .caption
                    .as_ref()
                    .map(|caption| caption.contains(term))
                    .unwrap_or(false)
                || image.alt.contains(term)
        })
}

/// Search all content of a mural by a text query
fn search_mural_all(mural: &Mural, term: &str) -> bool {
    search_mural_text(mural, term) || format!("{}", mural.year).contains(term)
}
