use crate::{
    kana_utils::{katakana_to_hiragana, romaji_to_katakana},
    DictionaryEntry,
};
use regex::Regex;
use std::collections::HashMap;

pub trait Query {
    fn searchable_terms(&self) -> Vec<&str>;
}

pub struct Dictionary<'a> {
    dict_map: HashMap<&'a str, Vec<&'a DictionaryEntry>>,
}

impl<'a> Dictionary<'a> {
    pub fn create(entries: &'a [DictionaryEntry]) -> Dictionary {
        let mut map: HashMap<&'a str, Vec<&'a DictionaryEntry>> = HashMap::new();

        for dict in entries {
            for key in dict.searchable_terms() {
                map.entry(key).or_default().push(dict);
            }
        }
        Dictionary { dict_map: map }
    }

    pub fn get(&self, key: &str) -> Vec<&'a DictionaryEntry> {
        self.dict_map
            .get(key)
            .map(|vec| vec.to_owned())
            .unwrap_or_default()
    }

    pub fn get_all(&self, keys: &[&str]) -> Vec<&'a DictionaryEntry> {
        keys.iter()
            .flat_map(|key| {
                self.dict_map
                    .get(key)
                    .map(|vec| vec.to_owned())
                    .unwrap_or_default()
            })
            .collect()
    }

    pub fn query(&self, raw_query: &str) -> Vec<&DictionaryEntry> {
        let clean_query = clean_query(raw_query);
        let (query, flags) = extract_flags(&clean_query);
        self.query_with_flags(&query, flags)
    }

    pub fn query_with_flags(&self, query: &str, flags: Vec<&str>) -> Vec<&DictionaryEntry> {
        let query_converted_to_katakana = romaji_to_katakana(query).ok();
        let query_converted_to_hiragana = query_converted_to_katakana
            .clone()
            .map(|katakana| katakana_to_hiragana(&katakana).expect("Should be valid katakana"));
        match (query_converted_to_hiragana, query_converted_to_katakana) {
            (Some(hiragana), Some(katakana))
                if self.worth_converting(query, &[&hiragana, &katakana]) =>
            {
                self.get_all(&[&hiragana, &katakana])
            }
            _ => self.get(query),
        }
        .into_iter()
        .filter(|item| {
            flags.is_empty()
                || flags.contains(&format!("#{}", &item.name().to_lowercase()).as_str())
        })
        .collect()
    }

    fn worth_converting(&self, query: &str, converted: &[&str]) -> bool {
        let num_result_normal = self.get(query).len();
        let num_result_converted = self.get_all(converted).len();
        num_result_converted > num_result_normal
    }
}

fn clean_query(query: &str) -> String {
    query
        .trim_matches(|c: char| c == '"' || c == '“' || c == '”' || c.is_whitespace())
        .to_lowercase()
}

fn extract_flags(query: &str) -> (String, Vec<&str>) {
    let pattern = Regex::new(r"#(kanji|word|name|radical)").unwrap();
    let flags: Vec<&str> = pattern.find_iter(query).map(|f| f.as_str()).collect();
    let cleaned_string: String = pattern.replace_all(query, "").trim().to_string();
    (cleaned_string, flags)
}
