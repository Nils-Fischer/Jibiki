use std::collections::HashMap;

use crate::build_dictionaries::{FromParsed, Key};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ParseTag {
    tag: String,
    category: String,
    _integer1: i32,
    description: String,
    _integer2: i32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Tag {
    tag: String,
    category: String,
    description: String,
}

impl Key<String> for Tag {
    fn key(&self) -> String {
        self.tag.clone()
    }
}

impl FromParsed<ParseTag> for Tag {
    fn from_parsed(parsed: ParseTag, _tags: Option<&HashMap<String, Tag>>) -> Self {
        Tag {
            tag: parsed.tag,
            category: parsed.category,
            description: parsed.description,
        }
    }
}
