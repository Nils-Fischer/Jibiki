use crate::{
    composite_dictionaries::{Kanji, Name, Radical, Word},
    kana_utils::{katakana_to_hiragana, romaji_to_katakana},
};
use anyhow::Result;
use std::collections::HashMap;

pub trait Query {
    fn queries(&self) -> Vec<&str>;
}

pub fn to_queriable_dict<'a, D: Query>(dicts: &'a [D]) -> HashMap<&'a str, Vec<&'a D>> {
    let mut map: HashMap<&'a str, Vec<&'a D>> = HashMap::new();

    for dict in dicts {
        for key in dict.queries() {
            map.entry(key).or_default().push(dict);
        }
    }
    map
}

pub struct QueriableDict<'a> {
    word_dict: HashMap<&'a str, Vec<&'a Word>>,
    kanji_dict: HashMap<&'a str, Vec<&'a Kanji>>,
    name_dict: HashMap<&'a str, Vec<&'a Name>>,
    radical_dict: HashMap<&'a str, Vec<&'a Radical>>,
}

impl<'a> QueriableDict<'a> {
    pub fn new(
        words: HashMap<&'a str, Vec<&'a Word>>,
        kanjis: HashMap<&'a str, Vec<&'a Kanji>>,
        names: HashMap<&'a str, Vec<&'a Name>>,
        radicals: HashMap<&'a str, Vec<&'a Radical>>,
    ) -> QueriableDict<'a> {
        QueriableDict {
            word_dict: words,
            kanji_dict: kanjis,
            name_dict: names,
            radical_dict: radicals,
        }
    }

    pub fn query(&self, query: &str) {
        if let Some(result) = self.word_dict.get(&query) {
            println!("{:#?}", result)
        }
        if let Some(result) = self.kanji_dict.get(&query) {
            println!("{:#?}", result)
        }
        if let Some(result) = self.name_dict.get(&query) {
            println!("{:#?}", result)
        }
        if let Some(result) = self.radical_dict.get(&query) {
            println!("{:#?}", result)
        }
    }

    pub fn get_by_romaji<D>(
        &self,
        dict: &'a HashMap<&'a str, Vec<&'a D>>,
        query: &str,
    ) -> Result<Option<Vec<&D>>> {
        let katakana_query = romaji_to_katakana(query)?;
        let hiragana_query = katakana_to_hiragana(&katakana_query)?;
        let katakana_results = dict
            .get(katakana_query.as_str())
            .cloned()
            .unwrap_or_default();
        let hiragana_results = dict
            .get(hiragana_query.as_str())
            .cloned()
            .unwrap_or_default();
        let combined_results: Vec<&D> = katakana_results
            .into_iter()
            .chain(hiragana_results)
            .collect();
        Ok(match combined_results.is_empty() {
            true => None,
            false => Some(combined_results),
        })
    }
}
