use anyhow::Result;
use std::cmp::min;

use crate::{Dictionary, DictionaryEntry};

const LONGEST_POSSIBLE_WORD: usize = 42;

fn extract_next_word<'a>(
    dict: &'a Dictionary,
    sentence: &[char],
    start_index: usize,
) -> Option<(Vec<&'a DictionaryEntry>, usize)> {
    let range = start_index..min(sentence.len(), start_index + LONGEST_POSSIBLE_WORD);
    for end in range.rev() {
        let substring: String = sentence[start_index..=end].iter().collect();
        let query_result = {
            let word_result = dict.query_with_flags(&substring, vec!["#word"]);
            if word_result.is_empty() {
                dict.query_with_flags(&substring, vec!["#name"])
            } else {
                word_result
            }
        };
        if !query_result.is_empty() {
            return Some((query_result, end));
        }
    }
    None
}

pub fn extract_all_words<'a>(
    dict: &'a Dictionary,
    sentence: &str,
) -> Option<Vec<Vec<&'a DictionaryEntry>>> {
    let mut index = 0;
    let mut results: Vec<Vec<&DictionaryEntry>> = Vec::new();
    let char_sentence: Vec<char> = sentence.chars().collect();
    while index != sentence.chars().count() {
        if let Some((result, i)) = extract_next_word(dict, &char_sentence, index) {
            index = i + 1;
            results.push(result);
        } else {
            return None;
        }
    }
    Some(results)
}

fn parse_example_sentences_from_tsv(file_path: &str) -> Result<String> {
    todo!();
}
