use std::collections::HashMap;

use crate::build_dictionaries::FromParsed;
use crate::build_dictionaries::Key;
use crate::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ParseInnocent {
    vocabulary: String,
    _freq: String,
    frequency: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Innocent {
    pub vocabulary: String,
    pub frequency: u32,
}

impl Key<String> for Innocent {
    fn key(&self) -> String {
        self.vocabulary.clone()
    }
}

impl FromParsed<ParseInnocent> for Innocent {
    fn from_parsed(parsed: ParseInnocent, _tags: Option<&HashMap<String, Tag>>) -> Self {
        Innocent {
            vocabulary: parsed.vocabulary,
            frequency: parsed.frequency,
        }
    }
}
