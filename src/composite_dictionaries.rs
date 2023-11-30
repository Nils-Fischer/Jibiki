use crate::{
    innocent_dictionary::Innocent,
    jmdict_dictionary::Jmdict,
    kanji_dictionaries::{Kanjidic, Kanjium, Pitches},
    radical_dictionaries::{Krad, Radk},
    tag::Tag,
};
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
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
    pub fn from(jmdict: &Jmdict, innocent: Option<&Innocent>, kanjium: Option<&Kanjium>) -> Word {
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct Name {
    name: String,
    reading: String,
    tags: HashMap<String, Tag>,
    translations: Vec<String>,
    id: u32,
}

impl Name {
    pub fn from(jmnedict: &Jmdict) -> Name {
        Name {
            name: jmnedict.vocabulary.clone(),
            reading: jmnedict.reading.clone(),
            tags: jmnedict.tags.clone(),
            translations: jmnedict.meanings.clone(),
            id: jmnedict.id,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
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
    pub fn from(kanjidic: &Kanjidic, innocent: Option<&Innocent>, krad: Option<&Krad>) -> Kanji {
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct Radical {
    radical: String,
    strokes: u8,
    kanji: String,
}

impl Radical {
    pub fn from(radk: &Radk) -> Radical {
        Radical {
            radical: radk.radical.clone(),
            strokes: radk.strokes,
            kanji: radk.kanji.chars().collect(),
        }
    }
}
