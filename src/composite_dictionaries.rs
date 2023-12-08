use crate::{
    basic_dictionaries::*,
    build_dictionaries::export_dicts_as_bin,
    dict_paths::{
        ExportPath, KANJIS_EXPORT_PATH, NAMES_EXPORT_PATH, RADICALS_EXPORT_PATH, WORDS_EXPORT_PATH,
    },
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    vocabulary: String,
    reading: String,
    tags: HashMap<String, Tag>,
    meanings: Vec<String>,
    id: u32,
    frequency: Option<u32>,
    pitches: Option<Vec<Pitches>>,
}

impl Word {
    pub fn from(jmdict: Jmdict, innocent: Option<&Innocent>, kanjium: Option<&Kanjium>) -> Word {
        Word {
            vocabulary: jmdict.vocabulary.clone(),
            reading: jmdict.reading.clone(),
            tags: jmdict.tags.clone(),
            meanings: jmdict.meanings.clone(),
            id: jmdict.id,
            frequency: innocent.map(|i| i.frequency),
            pitches: kanjium.map(|k| k.pitch.pitches.clone()),
        }
    }
}

impl ExportPath for Word {
    fn export_path(&self) -> String {
        WORDS_EXPORT_PATH.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    name: String,
    reading: String,
    tags: HashMap<String, Tag>,
    translations: Vec<String>,
    id: u32,
}

impl Name {
    pub fn from(jmnedict: Jmdict) -> Name {
        Name {
            name: jmnedict.vocabulary.clone(),
            reading: jmnedict.reading.clone(),
            tags: jmnedict.tags.clone(),
            translations: jmnedict.meanings.clone(),
            id: jmnedict.id,
        }
    }
}

impl ExportPath for Name {
    fn export_path(&self) -> String {
        NAMES_EXPORT_PATH.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kanji {
    kanji: String,
    kun_yomi: String,
    on_yomi: String,
    tags: HashMap<String, Tag>,
    meaning: Vec<String>,
    attributes: HashMap<String, String>,
    frequency: Option<u32>,
    radicals: Option<Vec<String>>,
}

impl Kanji {
    pub fn from(kanjidic: Kanjidic, innocent: Option<&Innocent>, krad: Option<&Krad>) -> Kanji {
        Kanji {
            kanji: kanjidic.kanji.clone(),
            kun_yomi: kanjidic.kun_yomi.clone(),
            on_yomi: kanjidic.on_yomi.clone(),
            tags: kanjidic.tags.clone(),
            meaning: kanjidic.meanings.clone(),
            attributes: kanjidic.attributes.clone(),
            frequency: innocent.map(|i| i.frequency),
            radicals: krad.map(|k| k.radicals.clone()),
        }
    }
}

impl ExportPath for Kanji {
    fn export_path(&self) -> String {
        KANJIS_EXPORT_PATH.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radical {
    radical: String,
    strokes: u8,
    kanji: String,
}

impl Radical {
    pub fn from(radk: Radk) -> Radical {
        Radical {
            radical: radk.radical.clone(),
            strokes: radk.strokes,
            kanji: radk.kanji.chars().collect(),
        }
    }
}

impl ExportPath for Radical {
    fn export_path(&self) -> String {
        RADICALS_EXPORT_PATH.to_string()
    }
}

pub enum CompositeDicts {
    Words(Vec<Word>),
    Names(Vec<Name>),
    Kanjis(Vec<Kanji>),
    Radicals(Vec<Radical>),
}

impl CompositeDicts {
    pub fn export_as_bin(&self) -> Result<()> {
        match self {
            CompositeDicts::Kanjis(kanjis) => export_dicts_as_bin(kanjis),
            CompositeDicts::Words(words) => export_dicts_as_bin(words),
            CompositeDicts::Names(names) => export_dicts_as_bin(names),
            CompositeDicts::Radicals(radicals) => export_dicts_as_bin(radicals),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            CompositeDicts::Kanjis(_) => "Kanjis",
            CompositeDicts::Words(_) => "Words",
            CompositeDicts::Names(_) => "Names",
            CompositeDicts::Radicals(_) => "Radicals",
        }
    }
}
