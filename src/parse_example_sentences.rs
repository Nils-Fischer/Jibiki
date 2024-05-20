use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    cmp::min,
    fs::File,
    io::{self, BufRead},
};

use crate::{
    sentence::{ParsedWord, Sentence},
    Dictionary, DictionaryEntry,
};

lazy_static! {
    static ref PARTICLES: String = "(は|が|を|に|へ|で|と|の|も|から|まで)".to_string();
    static ref LIKELY_ENDINGS: String = "(か|よ|ね|だ|だよ|だね|だよね)$".to_string();
    static ref NON_WORD_CHARS: Regex =
        Regex::new(r"[a-zA-Z\d・、。！？「」『』・〜（）《》〈〉【】　]+").unwrap();
    static ref KANJI_AND_KATAKANA: Regex = Regex::new(r"(\p{Katakana}+|\p{Han}+)").unwrap();
    static ref LIKELY_PARTICLES: Regex =
        Regex::new(format!(r"(?:^{}|{}$)", PARTICLES.as_str(), PARTICLES.as_str()).as_str())
            .unwrap();
    static ref CERTAIN_ENDINGS: Regex =
        Regex::new(r"(です|ですよ|ですね|ですよね|ですか)$").unwrap();
}

const LONGEST_POSSIBLE_WORD: usize = 42;

fn query_words_names<'a>(
    dict: &'a Dictionary,
    word: &str,
    strict: bool,
) -> Vec<&'a DictionaryEntry> {
    dict.query_with_flags(word, vec!["#word", "#name"])
        .into_iter()
        .filter(|entry| strict || entry.primary_word() == word)
        .sorted_by_key(|entry| entry.id().unwrap_or(u32::MAX))
        .sorted_by_key(|entry| entry.frequency().unwrap_or(u32::MAX))
        .collect()
}

pub fn extract_word_at_index<'a>(
    dict: &'a Dictionary,
    string: &str,
    start_index: usize,
    strict: bool,
) -> Option<(ParsedWord<'a>, usize)> {
    let chars: Vec<char> = string.chars().collect();
    let range = start_index..min(chars.len(), start_index + LONGEST_POSSIBLE_WORD);
    for end in range.rev() {
        let substring: String = chars[start_index..=end].iter().collect();
        let query_result = query_words_names(dict, &substring, strict);
        if !query_result.is_empty() {
            return Some((ParsedWord::create(substring, query_result), end));
        }
    }
    None
}

fn extract_words_sequentially<'a>(
    dict: &'a Dictionary,
    string: &str,
    strict: bool,
) -> Option<Vec<ParsedWord<'a>>> {
    let mut index = 0;
    let mut results: Vec<ParsedWord> = Vec::new();
    while index != string.chars().count() {
        if let Some((result, i)) = extract_word_at_index(dict, string, index, strict) {
            results.push(result);
            index = i + 1;
        } else {
            return None;
        }
    }
    Some(results)
}

fn extract_hiragana_words<'a>(dict: &'a Dictionary, string: &str) -> Option<Vec<ParsedWord<'a>>> {
    let (endings, remnants) = extract_by_pattern(&CERTAIN_ENDINGS, dict, string, true);
    let string = remnants
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or_default();
    todo!()
}

fn extract_by_pattern<'a>(
    pattern: &Regex,
    dict: &'a Dictionary,
    sentence: &str,
    strict: bool,
) -> (Vec<ParsedWord<'a>>, Vec<String>) {
    let result: Option<Vec<ParsedWord>> =
        pattern
            .find_iter(sentence)
            .try_fold(Vec::new(), |mut acc, mat| {
                let words = extract_words_sequentially(dict, mat.as_str(), strict)?;
                acc.extend(words);
                Some(acc)
            });
    match result {
        Some(words) => {
            let stripped_sentence: Vec<String> = LIKELY_PARTICLES
                .split(sentence)
                .map(|s| s.to_string())
                .collect();
            (words, stripped_sentence)
        }
        None => (Vec::new(), vec![sentence.to_string()]),
    }
}

fn extract_kanji_katakana_words<'a>(
    dict: &'a Dictionary,
    sentence: &str,
) -> (Vec<ParsedWord<'a>>, Vec<String>) {
    let mut remnants: Vec<String> = Vec::new();
    let mut extracted: Vec<ParsedWord> = Vec::new();
    let mut string: &str = sentence;
    while let Some(mat) = KANJI_AND_KATAKANA.find(string) {
        match extract_word_at_index(dict, string, mat.start(), false) {
            Some((parsed, _)) => {
                let (p1, p2) = string
                    .split_once(parsed.word.as_str())
                    .expect("String should contained the parsed word");
                remnants.push(p1.to_string());
                string = p2;
                extracted.push(parsed);
            }
            None => {
                let (p1, p2) = string
                    .split_once(mat.as_str())
                    .expect("String should contained the parsed word");
                string = p2;
                remnants.push(p1.to_string());
                extracted.push(ParsedWord::create(mat.as_str().to_string(), Vec::new()));
            }
        }
    }
    (
        extracted,
        remnants.into_iter().filter(|s| !s.is_empty()).collect(),
    )
}

pub fn parse_sentence<'a>(dict: &'a Dictionary, sentence: &str, id: u32) -> Sentence<'a> {
    let (extracted_kanji_katakana, substrings) = NON_WORD_CHARS.split(sentence).fold(
        (Vec::new(), Vec::new()),
        |(mut acc_extracted, mut acc_strings), string| {
            let (extracted, strings) = extract_kanji_katakana_words(dict, string);
            acc_extracted.extend(extracted);
            acc_strings.extend(strings);
            (acc_extracted, acc_strings)
        },
    );
    let extracted_hiragana = substrings
        .into_iter()
        .try_fold(Vec::new(), |mut acc, string| {
            let words = extract_hiragana_words(dict, &string)?;
            acc.extend(words);
            Some(acc)
        });
    let extracted = extracted_hiragana.map(|mut v| {
        v.extend(extracted_kanji_katakana);
        v
    });
    Sentence::create(sentence.to_string(), extracted, id)
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
