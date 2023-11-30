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
