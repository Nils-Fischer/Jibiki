use crate::{
    build_dictionaries::{FromParsed, Key},
    tag::Tag,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ParseKanjidic {
    kanji: String,
    kun_yomi: String,
    on_yomi: String,
    tags: String,
    meanings: Vec<String>,
    attributes: HashMap<String, String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Kanjidic {
    pub kanji: String,
    pub kun_yomi: String,
    pub on_yomi: String,
    pub tags: HashMap<String, Tag>,
    pub meanings: Vec<String>,
    pub attributes: HashMap<String, String>,
}

impl FromParsed<ParseKanjidic> for Kanjidic {
    fn from_parsed(parsed: ParseKanjidic, tags: Option<&HashMap<String, Tag>>) -> Self {
        Kanjidic {
            kanji: parsed.kanji,
            kun_yomi: parsed.kun_yomi,
            on_yomi: parsed.on_yomi,
            tags: {
                let tag_map = tags.unwrap_or_else(|| panic!("Expected tags are missing"));
                parsed
                    .tags
                    .split(' ')
                    .filter_map(|str| tag_map.get(str).map(|tag| (str.to_owned(), tag.to_owned())))
                    .collect()
            },
            meanings: parsed.meanings,
            attributes: parsed.attributes,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ParseKanjium {
    vocabulary: String,
    _name: String,
    pitch: ParsePitch,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ParsePitch {
    reading: String,
    pitches: Vec<ParsePitches>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ParsePitches {
    position: u16,
    tags: Option<Vec<String>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Kanjium {
    pub vocabulary: String,
    pub pitch: Pitch,
}

impl Key<String> for Kanjium {
    fn key(&self) -> String {
        self.vocabulary.clone()
    }
}

impl FromParsed<ParseKanjium> for Kanjium {
    fn from_parsed(parsed: ParseKanjium, tags: Option<&HashMap<String, Tag>>) -> Self {
        Kanjium {
            vocabulary: parsed.vocabulary,
            pitch: Pitch::from_parsed(parsed.pitch, tags),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Pitch {
    pub reading: String,
    pub pitches: Vec<Pitches>,
}

impl FromParsed<ParsePitch> for Pitch {
    fn from_parsed(parsed: ParsePitch, tags: Option<&HashMap<String, Tag>>) -> Self {
        Pitch {
            reading: parsed.reading,
            pitches: parsed
                .pitches
                .into_iter()
                .map(|pitch| Pitches::from_parsed(pitch, tags))
                .collect(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Pitches {
    position: u16,
    tags: HashMap<String, Tag>,
}

impl FromParsed<ParsePitches> for Pitches {
    fn from_parsed(parsed: ParsePitches, tags: Option<&HashMap<String, Tag>>) -> Self {
        Pitches {
            position: parsed.position,
            tags: {
                let tag_map = tags.unwrap_or_else(|| panic!("Expected tags are missing"));
                parsed
                    .tags
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|str| {
                        tag_map
                            .get(str.as_str())
                            .map(|tag| (str.to_owned(), tag.to_owned()))
                    })
                    .collect()
            },
        }
    }
}
