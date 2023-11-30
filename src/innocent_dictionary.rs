use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ParseInnocent {
    vocabulary: String,
    _freq: String,
    frequency: u32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Innocent {
    vocabulary: String,
    pub frequency: u32,
}
