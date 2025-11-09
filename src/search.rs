use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::data::Mural;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Search {
    min_year: Option<u16>,
    max_year: Option<u16>,
}

impl Search {
    pub fn apply<'a>(
        &self, murals: &'a HashMap<String, Mural>
    ) -> Vec<(&'a String, &'a Mural)> {
        murals
            .iter()
            .filter_map(|m| self.evaluate(m))
            .collect()
    }

    fn evaluate<'a>(
        &self, mural: (&'a String, &'a Mural)
    ) -> Option<(&'a String, &'a Mural)> {
        if let Some(min_year) = self.min_year && mural.1.year < min_year
            { return None }
        if let Some(max_year) = self.max_year && mural.1.year > max_year
            { return None }
        Some(mural)
    }
}
