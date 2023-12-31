use crate::{
    composite_dictionaries::{Kanji, Name, Radical, Word},
    kana_utils::{katakana_to_hiragana, romaji_to_katakana},
};
use anyhow::Result;
use itertools::Itertools;
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
        words: &'a [Word],
        kanjis: &'a [Kanji],
        names: &'a [Name],
        radicals: &'a [Radical],
    ) -> QueriableDict<'a> {
        QueriableDict {
            word_dict: to_queriable_dict(words),
            kanji_dict: to_queriable_dict(kanjis),
            name_dict: to_queriable_dict(names),
            radical_dict: to_queriable_dict(radicals),
        }
    }

    fn worth_converting(&self, query: &str, converted: &[&str]) -> bool {
        let normal_num = self
            .query_dict(&self.word_dict, &[query])
            .map(|results| results.len())
            .unwrap_or_default()
            + self
                .query_dict(&self.name_dict, &[query])
                .map(|results| results.len())
                .unwrap_or_default()
            + self
                .query_dict(&self.kanji_dict, &[query])
                .map(|results| results.len())
                .unwrap_or_default();
        let converted_num = self
            .query_dict(&self.word_dict, converted)
            .map(|results| results.len())
            .unwrap_or_default()
            + self
                .query_dict(&self.name_dict, converted)
                .map(|results| results.len())
                .unwrap_or_default()
            + self
                .query_dict(&self.kanji_dict, converted)
                .map(|results| results.len())
                .unwrap_or_default();
        converted_num > normal_num
    }

    pub fn query(&self, query: &str) {
        let converted_to_katakana = romaji_to_katakana(query).ok();
        let converted_to_hiragana = converted_to_katakana
            .clone()
            .map(|katakana| katakana_to_hiragana(&katakana).expect("Should be valid katakana"));
        match converted_to_hiragana {
            None => self.output_query_result(vec![query]),
            Some(hiragana) => {
                let katakana = &converted_to_katakana.unwrap();
                match self.worth_converting(query, &[&hiragana, &katakana]) {
                    true => {
                        println!(
                            "Searched for {}, you can also search for \"{}\"",
                            hiragana, query
                        );
                        self.output_query_by_romaji(query)
                            .expect("Should be valid kana");
                    }
                    false => {
                        println!("You can also search for {} or {}", hiragana, katakana);
                        self.output_query_result(vec![query]);
                    }
                }
            }
        }
    }

    pub fn output_query_result(&self, queries: Vec<&str>) {
        println!();
        if let Some(results) = self.query_dict(&self.word_dict, &queries) {
            for result in results {
                println!("{}", result);
            }
            println!();
        }
        let kanji_queries: Vec<&str> = queries.iter().flat_map(|query| query.split("")).collect();
        if let Some(results) = self.query_dict(&self.kanji_dict, &kanji_queries) {
            for result in results {
                println!("{}", result);
            }
            println!();
        }
        if let Some(results) = self.query_dict(&self.name_dict, &queries) {
            for result in results {
                println!("{}", result);
            }
            println!();
        }
        if let Some(results) = self.query_dict(&self.radical_dict, &queries) {
            for result in results {
                println!("{}", result);
            }
            println!();
        }
    }

    pub fn output_query_by_romaji(&self, query: &str) -> Result<()> {
        let katakana_query = romaji_to_katakana(query)?;
        let hiragana_query = katakana_to_hiragana(&katakana_query)?;
        self.output_query_result(vec![katakana_query.as_str(), hiragana_query.as_str()]);
        Ok(())
    }

    fn query_dict<D: std::cmp::Ord>(
        &self,
        dict: &'a HashMap<&'a str, Vec<&'a D>>,
        queries: &[&'a str],
    ) -> Option<Vec<&'a D>> {
        let results: Vec<&D> = queries
            .iter()
            .flat_map(|query| dict.get(clean_query(query)).cloned().unwrap_or_default())
            .sorted()
            .collect();
        match results.is_empty() {
            true => None,
            false => Some(results),
        }
    }
}

fn clean_query(query: &str) -> &str {
    query.trim_matches(|c: char| c == '"' || c == '“' || c == '”' || c.is_whitespace())
}
