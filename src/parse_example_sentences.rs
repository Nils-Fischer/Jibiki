use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::min,
    fs::File,
    io::{self, BufRead},
};

use crate::{sentence::ParsedWord, sentence::Sentence, Dictionary, DictionaryEntry};

const LONGEST_POSSIBLE_WORD: usize = 42;

fn query_and_filter<'a>(dict: &'a Dictionary, word: &str) -> Vec<&'a DictionaryEntry> {
    dict.query_with_flags(word, vec!["#word", "#name"])
        .into_iter()
        .filter(|entry| match entry {
            DictionaryEntry::Word(entry) => entry.vocabulary == word,
            DictionaryEntry::Name(entry) => entry.name == word,
            _ => false,
        })
        .sorted_by_key(|entry| entry.id().unwrap_or(u32::MAX))
        .sorted_by_key(|entry| entry.frequency().unwrap_or(u32::MAX))
        .collect()
}

fn extract_word<'a>(
    dict: &'a Dictionary,
    sentence: &[char],
    start_index: usize,
) -> Option<(Vec<&'a DictionaryEntry>, usize)> {
    let range = start_index..min(sentence.len(), start_index + LONGEST_POSSIBLE_WORD);
    for end in range.rev() {
        let substring: String = sentence[start_index..=end].iter().collect();
        let query_result = query_and_filter(dict, &substring);
        if !query_result.is_empty() {
            return Some((query_result, end));
        }
    }
    None
}

fn parse<'a>(dict: &'a Dictionary, string: &str) -> Option<Vec<ParsedWord<'a>>> {
    let mut index = 0;
    let mut results: Vec<ParsedWord> = Vec::new();
    let chars: Vec<char> = string.chars().collect();
    while index != string.chars().count() {
        if let Some((result, i)) = extract_word(dict, &chars, index) {
            let parsed_word = ParsedWord::create(chars[index..=i].iter().collect(), result);
            results.push(parsed_word);
            index = i + 1;
        } else {
            return None;
        }
    }
    Some(results)
}

pub fn parse_sentence<'a>(dict: &'a Dictionary, sentence: &str, id: u32) -> Sentence<'a> {
    let re = Regex::new(r"[、。！？「」『』・〜ー（）《》〈〉【】　]").unwrap();
    let results: Option<Vec<ParsedWord>> = re
        .split(sentence)
        .map(|string| parse(dict, string))
        .try_fold(Vec::new(), |mut acc, vec_opt| {
            if let Some(vec) = vec_opt {
                acc.extend(vec);
                Some(acc)
            } else {
                None
            }
        });
    Sentence::create(sentence.to_string(), results, id)
}

pub fn parse_example_sentences_from_tsv<'a>(
    file_path: &str,
    dict: &'a Dictionary,
) -> Result<Vec<Sentence<'a>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let mut parts = line.split('\t');
            let id = parts.next()?.parse::<u32>().ok()?;
            let sentence = parts.last()?.to_string();
            Some((sentence, id))
        })
        .map(|(sentence, id)| parse_sentence(dict, &sentence, id))
        .collect())
}
