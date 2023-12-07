use crate::build_dictionaries::FromParsed;
use crate::build_dictionaries::Key;
use serde::{Deserialize, Serialize};

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
    fn from_parsed(
        parsed: Krad,
        _tags: Option<&std::collections::HashMap<String, crate::tag::Tag>>,
    ) -> Self {
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
    fn from_parsed(
        parsed: Radk,
        _tags: Option<&std::collections::HashMap<String, crate::tag::Tag>>,
    ) -> Self {
        parsed
    }
}
