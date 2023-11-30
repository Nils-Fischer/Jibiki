use crate::tag::Tag;
use anyhow::Result;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

pub const JMDICT: &str = "resources/jmdict.bin";
pub const JMNEDICT: &str = "resources/jmnedict.bin";
pub const KANJIDIC: &str = "resources/kanjidic.bin";
pub const KANJIUM: &str = "resources/kanjium.bin";
pub const INNOCENT: &str = "resources/innocent.bin";
pub const KRAD: &str = "resources/krad.bin";
pub const RADK: &str = "resources/radk.bin";

pub fn load_json_dicts<P, D>(paths: Vec<&str>, tags: Option<&HashMap<&str, &Tag>>) -> Result<Vec<D>>
where
    P: for<'a> Deserialize<'a> + Send + Serialize,
    D: Send + for<'b> From<&'b P>,
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
                .map(|parsed| D::from(&parsed))
                .collect::<Vec<D>>()
        })
        .collect();
    Ok(dicts)
}

pub trait Key {
    fn key<K>(&self) -> K;
}

pub fn create_hash_map<K, V>(dicts: &Vec<V>) -> Result<HashMap<K, &V>>
where
    V: Key,
    K: std::cmp::Eq + std::hash::Hash,
{
    let map: HashMap<K, &V> = dicts.iter().map(|dict| (dict.key(), dict)).collect();
    Ok(map)
}
