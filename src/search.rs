use crate::data::Mural;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Search {
    min_year: Option<u16>,
    max_year: Option<u16>,
    year: Option<u16>,
    #[serde(default)]
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
        self.min_year.is_some() || self.max_year.is_some() || self.year.is_some() || self.by_decade
    }

    fn evaluate<'a>(&self, mural: (&'a String, &'a Mural)) -> Option<(&'a String, &'a Mural)> {
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
