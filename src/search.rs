use crate::data::{Data, Mural};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
#[serde(deny_unknown_fields, default)]
pub struct Search {
    // Generic search over text and metadata
    pub query: String,
    // Filter by associated text
    pub text: String,
    // Filter by tag
    pub tag: String,
    // Filter by artist
    pub artist: String,
    // Filter by year painted
    pub min_year: Option<u16>,
    pub max_year: Option<u16>,
    pub year: Option<u16>,
    pub by_decade: bool,
}

impl Search {
    /// Filter and order a set of murals
    pub fn apply<'a>(&self, data: &'a Data) -> Vec<(&'a String, &'a Mural)> {
        data.murals
            .iter()
            .filter_map(|m| self.evaluate(m, data))
            .collect()
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
            || !self.tag.is_empty()
            || !self.artist.is_empty()
            || self.min_year.is_some()
            || self.max_year.is_some()
            || self.year.is_some()
            || self.by_decade
    }

    /// Check if the search filters by tag only
    pub fn tag_only(&self) -> bool {
        !self.tag.is_empty()
            && self.artist.is_empty()
            && self.year.is_none()
            && self.no_non_header()
    }

    /// Check if the search filters by artist only
    pub fn artist_only(&self) -> bool {
        !self.artist.is_empty()
            && self.tag.is_empty()
            && self.year.is_none()
            && self.no_non_header()
    }

    /// Check if the search filters by year only
    pub fn year_only(&self) -> bool {
        self.year.is_some() && self.tag.is_empty() && self.artist.is_empty() && self.no_non_header()
    }

    /// Check if any parameters are set that don't get a special header
    fn no_non_header(&self) -> bool {
        self.query.is_empty()
            && self.text.is_empty()
            && self.min_year.is_none()
            && self.max_year.is_none()
    }

    fn evaluate<'a>(
        &self,
        mural: (&'a String, &'a Mural),
        data: &Data,
    ) -> Option<(&'a String, &'a Mural)> {
        // Filter by general query
        if !self.query.is_empty()
            && !self
                .query
                .split(' ')
                .all(|term| search_mural_all(mural.1, &term.to_ascii_lowercase(), data))
        {
            return None;
        }

        // Filter by tag
        if !self.tag.is_empty() && !mural.1.tags.iter().any(|tag| tag == &self.tag) {
            return None;
        }

        // Filter by artist
        if !self.artist.is_empty() && !mural.1.artists.iter().any(|artist| artist == &self.artist) {
            return None;
        }

        // Filter by text
        if !self.text.is_empty()
            && !self
                .text
                .split(' ')
                .all(|term| search_mural_text(mural.1, &term.to_ascii_lowercase()))
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
    mural.title.to_ascii_lowercase().contains(term)
        || mural.description.to_ascii_lowercase().contains(term)
        || mural.images.iter().any(|image| {
            image
                .caption
                .as_ref()
                .map(|caption| caption.to_ascii_lowercase().contains(term))
                .unwrap_or(false)
                || image.alt.to_ascii_lowercase().contains(term)
        })
}

/// Search all content of a mural by a text query
fn search_mural_all(mural: &Mural, term: &str, data: &Data) -> bool {
    search_mural_text(mural, term)
        || mural.tags.iter().any(|tag| {
            data.tags
                .get(tag)
                .unwrap()
                .name
                .to_ascii_lowercase()
                .contains(term)
        })
        || mural.artists.iter().any(|artist| {
            data.artists
                .get(artist)
                .unwrap()
                .name
                .to_ascii_lowercase()
                .contains(term)
        })
        || format!("{}", mural.year).contains(term)
}
