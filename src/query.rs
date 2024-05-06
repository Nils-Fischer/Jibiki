use crate::{
    composite_dictionaries::{Kanji, Name, Radical, Word},
    kana_utils::{katakana_to_hiragana, romaji_to_katakana, KANJI_CHARS},
};
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
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

    fn extract_flags(&self, query: &str) -> (String, Vec<String>) {
        let pattern = Regex::new(r"#(kanji|word|name|radical)").unwrap();
        let flags: Vec<String> = pattern
            .find_iter(query)
            .map(|f| f.as_str().to_string())
            .collect();
        let cleaned_string: String = pattern.replace_all(query, "").trim().to_string();
        (cleaned_string, flags)
    }

    pub fn query(&self, raw_query: &str) -> String {
        let (query, flags) = self.extract_flags(&raw_query.to_lowercase());
        println!("{}", query);
        println!("{:?}", flags);
        let converted_to_katakana = romaji_to_katakana(&query).ok();
        let converted_to_hiragana = converted_to_katakana
            .clone()
            .map(|katakana| katakana_to_hiragana(&katakana).expect("Should be valid katakana"));
        match converted_to_hiragana {
            None => self.generate_query_result(vec![&query], &flags),
            Some(hiragana) => {
                let katakana = &converted_to_katakana.unwrap();
                match self.worth_converting(&query, &[&hiragana, &katakana]) {
                    true => {
                        format!(
                            "Searched for {}, you can also search for \"{}\"\n{}",
                            hiragana,
                            query,
                            self.generate_romaji_query_result(&query, &flags)
                                .expect("Should be valid kana")
                        )
                    }
                    false => {
                        format!(
                            "You can also search for {} or {}\n{}",
                            hiragana,
                            katakana,
                            self.generate_query_result(vec![&query], &flags)
                        )
                    }
                }
            }
        }
    }

    pub fn generate_query_result(&self, queries: Vec<&str>, flags: &[String]) -> String {
        let mut output = String::new();
        output.push('\n');
        if flags.is_empty() || flags.contains(&"#word".to_string()) {
            if let Some(results) = self.query_dict(&self.word_dict, &queries) {
                for result in results {
                    output.push_str(&format!("{}\n", result));
                }
                output.push('\n');
            }
        }
        if flags.is_empty() || flags.contains(&"#kanji".to_string()) {
            let kanji_queries: Vec<&str> = queries
                .iter()
                .flat_map(|query| query.split(""))
                .filter(|char| KANJI_CHARS.is_match(char))
                .collect();
            if let Some(results) = self.query_dict(&self.kanji_dict, &kanji_queries) {
                for result in results {
                    output.push_str(&format!("{}\n", result));
                }
                output.push('\n');
            }
        }
        if flags.is_empty() || flags.contains(&"#name".to_string()) {
            if let Some(results) = self.query_dict(&self.name_dict, &queries) {
                for result in results {
                    output.push_str(&format!("{}\n", result));
                }
                output.push('\n');
            }
        }
        if flags.is_empty() || flags.contains(&"#radical".to_string()) {
            if let Some(results) = self.query_dict(&self.radical_dict, &queries) {
                for result in results {
                    output.push_str(&format!("{}\n", result));
                }
                output.push('\n');
            }
        }
        output
    }

    pub fn generate_romaji_query_result(&self, query: &str, flags: &Vec<String>) -> Result<String> {
        let katakana_query = romaji_to_katakana(query)?;
        let hiragana_query = katakana_to_hiragana(&katakana_query)?;
        Ok(self.generate_query_result(
            vec![katakana_query.as_str(), hiragana_query.as_str()],
            flags,
        ))
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
