use crate::data::Mural;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Search {
    min_year: Option<u16>,
    max_year: Option<u16>,
    year: Option<u16>,
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
        self.min_year.is_some() || self.max_year.is_some() || self.year.is_some()
    }

    fn evaluate<'a>(&self, mural: (&'a String, &'a Mural)) -> Option<(&'a String, &'a Mural)> {
        if let Some(min_year) = self.min_year
            && mural.1.year < min_year
        {
            return None;
        }
        if let Some(max_year) = self.max_year
            && mural.1.year > max_year
        {
            return None;
        }
        if let Some(year) = self.year
            && mural.1.year != year
        {
            return None;
        }
        Some(mural)
    }
}
