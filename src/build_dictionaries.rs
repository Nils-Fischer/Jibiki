use crate::{
    composite_dictionaries::{Kanji, Name, Radical, Word},
    innocent_dictionary::Innocent,
    jmdict_dictionary::Jmdict,
    kanji_dictionaries::{Kanjidic, Kanjium},
    radical_dictionaries::{Krad, Radk},
    tag::Tag,
};
use anyhow::Result;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

pub const WORDS: &str = "target/words.bin";
pub const NAMES: &str = "target/names.bin";
pub const KANJIS: &str = "target/kanjis.bin";
pub const RADICALS: &str = "target/radicals.bin";

pub trait FromParsed<P> {
    fn from_parsed(parsed: P, tags: Option<&HashMap<&str, Tag>>) -> Self;
}

pub fn load_json_dicts<P, D>(paths: Vec<&str>, tags: Option<&HashMap<&str, Tag>>) -> Result<Vec<D>>
where
    P: for<'a> Deserialize<'a> + Send + Serialize,
    D: Send + for<'b> FromParsed<&'b P>,
{
    let dicts: Vec<D> = paths
        .into_par_iter()
        .flat_map(|path| {
            let file: File = File::open(path)
                .unwrap_or_else(|e| panic!("Couldn't open file: {}, reason: {}", path, e));
            let parse_results: Vec<P> = serde_json::from_reader(file)
                .unwrap_or_else(|e| panic!("Couldn't parse json file: {}, reason: {}", path, e));
            parse_results
                .into_iter()
                .map(|parsed| D::from_parsed(&parsed, tags))
                .collect::<Vec<D>>()
        })
        .collect();
    Ok(dicts)
}

fn assemble_composite_dictionaries(
    jmdicts: Vec<Jmdict>,
    jmnedicts: Vec<Jmdict>,
    kanjium: Vec<Kanjium>,
    kanjidic: Vec<Kanjidic>,
    innocent: Vec<Innocent>,
    krad: Vec<Krad>,
    radk: Vec<Radk>,
) -> (Vec<Kanji>, Vec<Word>, Vec<Name>, Vec<Radical>) {
    let innocent_map: HashMap<String, Innocent> = innocent
        .into_iter()
        .map(|entry| (entry.vocabulary.clone(), entry))
        .collect();
    let krad_map: HashMap<String, Krad> = krad
        .into_iter()
        .map(|entry| (entry.kanji.clone(), entry))
        .collect();
    let kanjium_map: HashMap<String, Kanjium> = kanjium
        .into_iter()
        .map(|entry| (entry.vocabulary.clone(), entry))
        .collect();
    let kanji_dicts: Vec<Kanji> = assemble_kanji_dicts(kanjidic, &innocent_map, &krad_map);
    let word_dicts: Vec<Word> = assemble_word_dicts(jmdicts, &innocent_map, &kanjium_map);
    let name_dicts: Vec<Name> = assemble_name_dicts(jmnedicts);
    let radical_dicts: Vec<Radical> = assemble_radical_dicts(radk);
    (kanji_dicts, word_dicts, name_dicts, radical_dicts)
}

fn assemble_word_dicts(
    jmdicts: Vec<Jmdict>,
    innocent_map: &HashMap<String, Innocent>,
    kanjium_map: &HashMap<String, Kanjium>,
) -> Vec<Word> {
    jmdicts
        .into_iter()
        .map(|entry| {
            let innocent_value = innocent_map.get(entry.vocabulary.as_str());
            let kanjium_value = kanjium_map.get(entry.vocabulary.as_str());
            Word::from(entry, innocent_value, kanjium_value)
        })
        .collect()
}

fn assemble_name_dicts(jmnedicts: Vec<Jmdict>) -> Vec<Name> {
    jmnedicts.into_iter().map(Name::from).collect()
}

fn assemble_kanji_dicts(
    kanjidics: Vec<Kanjidic>,
    innocent_map: &HashMap<String, Innocent>,
    krad_map: &HashMap<String, Krad>,
) -> Vec<Kanji> {
    /*
     */
    kanjidics
        .into_iter()
        .map(|entry| {
            let innocent_value = innocent_map.get(entry.kanji.as_str());
            let krad_value = krad_map.get(entry.kanji.as_str());
            Kanji::from(entry, innocent_value, krad_value)
        })
        .collect()
}

fn assemble_radical_dicts(radk: Vec<Radk>) -> Vec<Radical> {
    radk.into_iter().map(Radical::from).collect()
}
