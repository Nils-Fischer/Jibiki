use crate::{
    adjective_conjugation_utils::generate_all_adjective_conjugations,
    basic_dictionaries::*,
    build_dictionaries::Key,
    query::Query,
    verb_conjugation_utils::{generate_all_verb_conjugations, ConjugatedWord},
};
use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Display},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    vocabulary: String,
    reading: String,
    meanings: Vec<String>,
    tags: HashMap<String, Tag>,
    id: u32,
    frequency: Option<u32>,
    pitches: Option<Vec<Pitches>>,
    conjugations: Vec<ConjugatedWord>,
}

impl Key<u32> for Word {
    fn key(&self) -> u32 {
        self.id
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", bold("Word:"), self.vocabulary)?;
        writeln!(f, "{} {}", bold("Reading:"), self.reading)?;
        writeln!(f, "{}", bold("Meanings:"))?;
        for (index, meaning) in self.meanings.iter().enumerate() {
            writeln!(f, "{}. {}", index + 1, meaning)?;
        }
        writeln!(
            f,
            "{} {}",
            bold("Conjugations:"),
            self.conjugations
                .iter()
                .map(|conj| &conj.kanji_form)
                .join("、")
        )?;
        let tags = self
            .tags
            .values()
            .map(|tag| tag.description.clone())
            .join(", ");
        if let Some(freq) = self.frequency {
            writeln!(f, "{} {}", bold("Frequency:"), freq)?;
        }
        /* if let Some(pitches) = self.pitches.clone() {
            writeln!(f, "{:#?}", pitches)?;
        } */
        writeln!(f, "{} {}", bold("ID:"), self.id)?;
        if !tags.is_empty() {
            writeln!(f, "{}", tags)?;
        }
        Ok(())
    }
}

impl Query for Word {
    fn searchable_terms(&self) -> Vec<&str> {
        self.meanings
            .iter()
            .flat_map(|meaning| meaning.split(", "))
            .chain(std::iter::once(self.vocabulary.as_str()))
            .chain(std::iter::once(self.reading.as_str()))
            .chain(self.tags.keys().map(AsRef::as_ref))
            .chain(self.conjugations.iter().map(|con| &*con.kana_form))
            .chain(self.conjugations.iter().map(|con| &*con.kanji_form))
            .collect()
    }
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
            conjugations: {
                generate_all_verb_conjugations(
                    &jmdict.vocabulary,
                    &jmdict.reading,
                    jmdict.tags.keys().collect(),
                )
                .or(generate_all_adjective_conjugations(
                    &jmdict.vocabulary,
                    &jmdict.reading,
                    jmdict.tags.keys().collect(),
                ))
                .unwrap_or_default()
            },
        }
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.frequency.unwrap_or(self.id) == other.frequency.unwrap_or(other.id)
    }
}

impl Eq for Word {}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency
            .unwrap_or(self.id)
            .cmp(&other.frequency.unwrap_or(other.id))
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

impl Key<u32> for Name {
    fn key(&self) -> u32 {
        self.id
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", bold("Name:"), self.name)?;
        writeln!(f, "{} {}", bold("Reading:"), self.reading)?;
        writeln!(
            f,
            "{} {}",
            bold("Translations:"),
            self.translations.iter().join(", ")
        )?;
        let tags = self
            .tags
            .values()
            .map(|tag| tag.description.clone())
            .join(", ");
        if !tags.is_empty() {
            writeln!(f, "{}", tags)?;
        }
        writeln!(f, "{}, {}", bold("ID:"), self.id)
    }
}

impl Query for Name {
    fn searchable_terms(&self) -> Vec<&str> {
        self.translations
            .iter()
            .map(AsRef::as_ref)
            .chain(std::iter::once(self.name.as_str()))
            .chain(std::iter::once(self.reading.as_str()))
            .collect()
    }
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

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Name {}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kanji {
    kanji: String,
    on_yomi: Vec<String>,
    kun_yomi: Vec<String>,
    meanings: Vec<String>,
    strokes: u8,
    id: u32,
    frequency: Option<u32>,
    jlpt: Option<u8>,
    grade: Option<u8>,
    radicals: Option<Vec<String>>,
    tags: HashMap<String, Tag>,
    attributes: HashMap<String, String>,
}

impl fmt::Display for Kanji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", bold("Kanji:"), self.kanji)?;
        writeln!(
            f,
            "{}, {}",
            bold("Meanings:"),
            self.meanings.iter().join(", ")
        )?;
        writeln!(f, "{} {}", bold("Kun:"), self.kun_yomi.iter().join("、"))?;
        writeln!(f, "{} {}", bold("On:"), self.on_yomi.iter().join("、"))?;
        writeln!(f, "{} {}", bold("Strokes:"), self.strokes)?;
        if let Some(frequency) = self.frequency {
            writeln!(f, "{}, {}", bold("Frequency:"), frequency)?;
        }
        if let Some(jlpt) = self.jlpt {
            writeln!(f, "{} level N{}", bold("JLPT"), jlpt)?;
        }
        if let Some(grade) = self.grade {
            writeln!(f, "Taught in {} {}", bold("grade"), grade)?;
        }
        if let Some(radicals) = self.radicals.clone() {
            writeln!(f, "{} {}", bold("Radicals:"), radicals.iter().join("、"))?;
        }
        let tags = self
            .tags
            .values()
            .map(|tag| tag.description.clone())
            .join(", ");
        for (attribute, value) in self.attributes.iter() {
            writeln!(f, "{}: {}", bold(attribute), value)?;
        }
        if !tags.is_empty() {
            writeln!(f, "{}", tags)?;
        }
        writeln!(f, "{} {}", bold("ID:"), self.id)?;
        Ok(())
    }
}

impl Key<u32> for Kanji {
    fn key(&self) -> u32 {
        self.id
    }
}

impl Query for Kanji {
    fn searchable_terms(&self) -> Vec<&str> {
        self.meanings
            .iter()
            .map(AsRef::as_ref)
            .chain(std::iter::once(self.kanji.as_str()))
            .chain(self.kun_yomi.iter().map(AsRef::as_ref))
            .chain(self.on_yomi.iter().map(AsRef::as_ref))
            .collect()
    }
}

impl Kanji {
    pub fn from(kanjidic: Kanjidic, innocent: Option<&Innocent>, krad: Option<&Krad>) -> Kanji {
        Kanji {
            kanji: kanjidic.kanji.clone(),
            kun_yomi: kanjidic
                .kun_yomi
                .split_whitespace()
                .map(String::from)
                .collect(),
            on_yomi: kanjidic
                .on_yomi
                .split_whitespace()
                .map(String::from)
                .collect(),
            tags: kanjidic.tags.clone(),
            meanings: kanjidic.meanings.clone(),
            frequency: innocent.map(|i| i.frequency),
            radicals: krad.map(|k| k.radicals.clone()),
            strokes: kanjidic
                .attributes
                .get("strokes")
                .and_then(|num| num.parse().ok())
                .unwrap(),
            id: kanjidic
                .attributes
                .get("ucs")
                .and_then(|ucs| u32::from_str_radix(ucs, 16).ok())
                .unwrap(),
            jlpt: kanjidic
                .attributes
                .get("jlpt")
                .and_then(|level| level.parse().ok()),
            grade: kanjidic
                .attributes
                .get("grade")
                .and_then(|num| num.parse().ok()),
            attributes: kanjidic
                .attributes
                .clone()
                .extract_if(|key, _| !["strokes", "ucs", "jlpt", "grade"].contains(&key.as_str()))
                .collect(),
        }
    }
}

impl PartialEq for Kanji {
    fn eq(&self, other: &Self) -> bool {
        self.frequency.unwrap_or(self.id) == other.frequency.unwrap_or(other.id)
    }
}

impl Eq for Kanji {}

impl PartialOrd for Kanji {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Kanji {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency
            .unwrap_or(self.id)
            .cmp(&other.frequency.unwrap_or(other.id))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Radical {
    radical: String,
    strokes: u8,
    kanji: String,
}

impl fmt::Display for Radical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", bold("Radical:"), self.radical)?;
        writeln!(f, "{} {}", bold("Strokes:"), self.strokes)?;
        writeln!(f, "{}, {}", bold("Part of:"), self.kanji.chars().join("、"))
    }
}

impl Query for Radical {
    fn searchable_terms(&self) -> Vec<&str> {
        vec![self.radical.as_str()]
    }
}

impl Key<String> for Radical {
    fn key(&self) -> String {
        self.radical.clone()
    }
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

impl PartialEq for Radical {
    fn eq(&self, other: &Self) -> bool {
        self.strokes == other.strokes
    }
}

impl Eq for Radical {}

impl PartialOrd for Radical {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Radical {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strokes.cmp(&other.strokes)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DictionaryEntry {
    Word(Word),
    Name(Name),
    Kanji(Kanji),
    Radical(Radical),
}

impl Query for DictionaryEntry {
    fn searchable_terms(&self) -> Vec<&str> {
        match self {
            DictionaryEntry::Kanji(entry) => entry.searchable_terms(),
            DictionaryEntry::Word(entry) => entry.searchable_terms(),
            DictionaryEntry::Name(entry) => entry.searchable_terms(),
            DictionaryEntry::Radical(entry) => entry.searchable_terms(),
        }
    }
}

impl Display for DictionaryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DictionaryEntry::Kanji(entry) => entry.fmt(f),
            DictionaryEntry::Word(entry) => entry.fmt(f),
            DictionaryEntry::Name(entry) => entry.fmt(f),
            DictionaryEntry::Radical(entry) => entry.fmt(f),
        }
    }
}

impl DictionaryEntry {
    pub fn name(&self) -> &str {
        match self {
            DictionaryEntry::Kanji(_) => "Kanji",
            DictionaryEntry::Word(_) => "Word",
            DictionaryEntry::Name(_) => "Name",
            DictionaryEntry::Radical(_) => "Radical",
        }
    }
}

pub fn export_vec_as_bin<D: Serialize>(vec: &Vec<D>, path: &str) -> Result<()> {
    let encoded: Vec<u8> = bincode::serialize(&vec)?;
    std::fs::write(path, encoded)?;
    Ok(())
}

fn bold(str: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", str)
}
