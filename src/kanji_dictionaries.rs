use crate::tag::Tag;
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
    vocabulary: String,
    pub pitch: Pitch,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Pitch {
    reading: String,
    pub pitches: Vec<Pitches>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Pitches {
    position: u16,
    tags: Option<HashMap<String, Tag>>,
}
