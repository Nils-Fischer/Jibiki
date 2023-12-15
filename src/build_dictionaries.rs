use crate::{
    basic_dictionaries::*,
    composite_dictionaries::{CompositeDicts, Kanji, Name, Radical, Word},
    dictionary_paths::*,
};
use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

pub trait FromParsed<P> {
    fn from_parsed(parsed: P, tags: Option<&HashMap<String, Tag>>) -> Self;
}

fn load_json_dicts<P, D>(paths: Vec<String>, tags: Option<HashMap<String, Tag>>) -> Result<Vec<D>>
where
    P: for<'a> Deserialize<'a> + Send + Serialize,
    D: Send + FromParsed<P>,
{
    let dicts: Vec<D> = paths
        .into_par_iter()
        .flat_map(|path| {
            let file: File = File::open(&path)
                .unwrap_or_else(|e| panic!("Couldn't open file: {}, reason: {}", path, e));
            let parse_results: Vec<P> = serde_json::from_reader(file)
                .unwrap_or_else(|e| panic!("Couldn't parse json file: {}, reason: {}", path, e));
            parse_results
                .into_iter()
                .map(|parsed| D::from_parsed(parsed, tags.as_ref()))
                .collect::<Vec<D>>()
        })
        .collect();
    Ok(dicts)
}

pub trait Key<K> {
    fn key(&self) -> K;
}

pub fn hashmap_from_dicts<K, D>(dicts: Vec<D>) -> HashMap<K, D>
where
    D: Key<K>,
    K: std::hash::Hash + Eq,
{
    dicts
        .into_iter()
        .map(|entry| (entry.key(), entry))
        .collect()
}

#[allow(dead_code)]
pub fn hashmap_of_dicts<K, D>(dicts: &[D]) -> HashMap<K, &D>
where
    D: Key<K>,
    K: std::hash::Hash + Eq,
{
    dicts.iter().map(|entry| (entry.key(), entry)).collect()
}

pub fn build_composite_dicts() -> Result<()> {
    let jmdict_tags: HashMap<String, Tag> =
        hashmap_from_dicts(load_json_dicts(jmdict_tag_paths(), None)?);
    let jmnedict_tags: HashMap<String, Tag> =
        hashmap_from_dicts(load_json_dicts(jmnedict_tag_paths(), None)?);
    let kanjidic_tags: HashMap<String, Tag> =
        hashmap_from_dicts(load_json_dicts(kanjidic_tag_paths(), None)?);
    let kanjium_tags: HashMap<String, Tag> =
        hashmap_from_dicts(load_json_dicts(kanjium_tag_paths(), None)?);
    let jmdicts: Vec<Jmdict> = load_json_dicts(jmdict_dict_paths(), Some(jmdict_tags))?;
    let jmnedicts: Vec<Jmdict> = load_json_dicts(jmnedict_dict_paths(), Some(jmnedict_tags))?;
    let kanjium: Vec<Kanjium> = load_json_dicts(kanjium_dict_paths(), Some(kanjium_tags))?;
    let kanjidic: Vec<Kanjidic> = load_json_dicts(kanjidic_dict_paths(), Some(kanjidic_tags))?;
    let innocent_kanji: Vec<Innocent> = load_json_dicts(innocent_kanji_dict_paths(), None)?;
    let innocent_vocab: Vec<Innocent> = load_json_dicts(innocent_vocab_dict_paths(), None)?;
    let krad: Vec<Krad> = load_json_dicts(krad_dict_paths(), None)?;
    let radk: Vec<Radk> = load_json_dicts(radk_dict_paths(), None)?;
    let composite_dicts: Vec<CompositeDicts> = assemble_composite_dicts(
        jmdicts,
        jmnedicts,
        kanjium,
        kanjidic,
        innocent_kanji,
        innocent_vocab,
        krad,
        radk,
    );
    for dicts in composite_dicts.iter() {
        match dicts.export_as_bin() {
            Ok(_) => println!("Succesfully exported {}!", dicts.name()),
            Err(e) => panic!("Failed to export {}, reason: {}", dicts.name(), e),
        }
    }
    Ok(())
}

pub fn export_dicts_as_bin<D: Serialize + ExportPath>(dicts: &Vec<D>) -> Result<()> {
    let encoded: Vec<u8> = bincode::serialize(&dicts)?;
    std::fs::write(dicts[0].export_path(), encoded)?;
    Ok(())
}

fn assemble_composite_dicts(
    jmdicts: Vec<Jmdict>,
    jmnedicts: Vec<Jmdict>,
    kanjium: Vec<Kanjium>,
    kanjidic: Vec<Kanjidic>,
    innocent_kanji: Vec<Innocent>,
    innocent_vocab: Vec<Innocent>,
    krad: Vec<Krad>,
    radk: Vec<Radk>,
) -> Vec<CompositeDicts> {
    let innocent_vocab_map: HashMap<String, Innocent> = hashmap_from_dicts(innocent_vocab);
    let innocent_kanji_map: HashMap<String, Innocent> = hashmap_from_dicts(innocent_kanji);
    let krad_map: HashMap<String, Krad> = hashmap_from_dicts(krad);
    let kanjium_map: HashMap<String, Kanjium> = hashmap_from_dicts(kanjium);
    let kanji_dicts: Vec<Kanji> = assemble_kanji_dicts(kanjidic, &innocent_kanji_map, &krad_map);
    let word_dicts: Vec<Word> = assemble_word_dicts(jmdicts, &innocent_vocab_map, &kanjium_map);
    let name_dicts: Vec<Name> = assemble_name_dicts(jmnedicts);
    let radical_dicts: Vec<Radical> = assemble_radical_dicts(radk);
    vec![
        CompositeDicts::Kanjis(kanji_dicts),
        CompositeDicts::Words(word_dicts),
        CompositeDicts::Names(name_dicts),
        CompositeDicts::Radicals(radical_dicts),
    ]
}

fn assemble_word_dicts(
    jmdicts: Vec<Jmdict>,
    innocent_map: &HashMap<String, Innocent>,
    kanjium_map: &HashMap<String, Kanjium>,
) -> Vec<Word> {
    jmdicts
        .into_iter()
        .group_by(|word| word.key())
        .into_iter()
        .map(|(_, group)| {
            let mut words = group.into_iter().peekable();
            let first = words.peek().unwrap();
            Jmdict {
                vocabulary: first.vocabulary.to_owned(),
                reading: first.reading.to_owned(),
                romaji: first.romaji.to_owned(),
                tags: first.tags.to_owned(),
                id: first.id,
                meanings: words
                    .map(|vec| vec.meanings.join(", "))
                    .collect::<HashSet<String>>()
                    .into_iter()
                    .collect(),
            }
        })
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
