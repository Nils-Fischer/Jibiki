use crate::build_dictionaries::FromParsed;
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
    vocabulary: String,
    pub frequency: u32,
}

impl FromParsed<ParseInnocent> for Innocent {
    fn from_parsed(
        parsed: ParseInnocent,
        _tags: Option<&std::collections::HashMap<&str, Tag>>,
    ) -> Self {
        Innocent {
            vocabulary: parsed.vocabulary,
            frequency: parsed.frequency,
        }
    }
}
