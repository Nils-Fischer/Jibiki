use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Krad {
    pub kanji: String,
    pub radicals: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Radk {
    pub radical: String,
    pub strokes: u8,
    pub kanji: String,
}
