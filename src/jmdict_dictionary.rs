use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Jmdict {
    pub vocabulary: String,
    pub reading: String,
    pub romaji: String,
    pub tags: HashMap<String, Tag>,
    pub meanings: Vec<String>,
    pub id: u32,
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
