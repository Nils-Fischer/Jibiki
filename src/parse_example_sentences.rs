use anyhow::Result;

use crate::{Dictionary, DictionaryEntry};

fn extract_words<'a>(sentence: &str, dict: Dictionary) -> Option<Vec<&'a DictionaryEntry>> {
    for length in (1..=sentence.len()).rev() {
        let substring = &sentence[0..length];
        let query_result = dict.query_with_flags(substring, vec!["word", "name"]);
        if !query_result.is_empty() {
            return todo!();
        }
    }
    None
}

fn parse_example_sentences_from_tsv(file_path: &str) -> Result<String> {
    todo!();
}
