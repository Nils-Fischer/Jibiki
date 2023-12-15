use crate::build_dictionaries::FromParsed;
use crate::build_dictionaries::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
pub enum BasicDicts {
    Jmdicts(Vec<Jmdict>),
    Jmnedicts(Vec<Jmdict>),
    Kanjiums(Vec<Kanjium>),
    Kanjidics(Vec<Kanjidic>),
    InnocentKanjis(Vec<Innocent>),
    InnocentVocabs(Vec<Innocent>),
    Krads(Vec<Krad>),
    Radks(Vec<Radk>),
}

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

impl Key<u32> for Jmdict {
    fn key(&self) -> u32 {
        self.id
    }
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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Krad {
    pub kanji: String,
    pub radicals: Vec<String>,
}

impl Key<String> for Krad {
    fn key(&self) -> String {
        self.kanji.clone()
    }
}

impl FromParsed<Krad> for Krad {
    fn from_parsed(parsed: Krad, _tags: Option<&std::collections::HashMap<String, Tag>>) -> Self {
        parsed
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Radk {
    pub radical: String,
    pub strokes: u8,
    pub kanji: String,
}

impl FromParsed<Radk> for Radk {
    fn from_parsed(parsed: Radk, _tags: Option<&std::collections::HashMap<String, Tag>>) -> Self {
        parsed
    }
}

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
