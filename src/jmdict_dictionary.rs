use crate::{build_dictionaries::FromParsed, tag::Tag};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ParseJmdict {
    vocabulary: String,
    reading: String,
    romaji: String,
    tags: String,
    meanings: Vec<String>,
    id: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Jmdict {
    pub vocabulary: String,
    pub reading: String,
    pub romaji: String,
    pub tags: HashMap<String, Tag>,
    pub meanings: Vec<String>,
    pub id: u32,
}

impl FromParsed<ParseJmdict> for Jmdict {
    fn from_parsed(parsed: ParseJmdict, tags: Option<&HashMap<String, Tag>>) -> Self {
        Jmdict {
            vocabulary: parsed.vocabulary,
            reading: parsed.reading,
            romaji: parsed.romaji,
            tags: {
                let tag_map = tags.unwrap_or_else(|| panic!("Expected tags are missing"));
                parsed
                    .tags
                    .split(' ')
                    .filter_map(|str| tag_map.get(str).map(|tag| (str.to_owned(), tag.to_owned())))
                    .collect()
            },
            meanings: parsed.meanings,
            id: parsed.id,
        }
    }
}
