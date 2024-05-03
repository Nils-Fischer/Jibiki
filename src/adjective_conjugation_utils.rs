use crate::verb_conjugation_utils::{ConjugatedWord, GrammaticalForm};
use lazy_static::lazy_static;
use std::collections::HashSet;

pub fn generate_all_adjective_conjugations(
    adjective_kanji_form: &str,
    adjective_reading_form: &str,
    tags: Vec<&String>,
) -> Option<Vec<ConjugatedWord>> {
    if !tags.iter().any(|s| s.starts_with('a')) || adjective_kanji_form.is_empty() {
        None
    } else if let Some(category) = tags.into_iter().find(|&tag| ADJECTIVE_TAGS.contains(tag)) {
        let adjective_kana_form = match adjective_reading_form {
            "" => adjective_kanji_form,
            _ => adjective_reading_form,
        };
        Some(vec![
            ConjugatedWord {
                kanji_form: negative(adjective_kanji_form, category),
                kana_form: negative(adjective_kana_form, category),
                grammatical_forms: vec![GrammaticalForm::Negative],
            },
            ConjugatedWord {
                kanji_form: past(adjective_kanji_form, category),
                kana_form: past(adjective_kana_form, category),
                grammatical_forms: vec![GrammaticalForm::Past],
            },
            ConjugatedWord {
                kanji_form: te_form(adjective_kanji_form, category),
                kana_form: te_form(adjective_kana_form, category),
                grammatical_forms: vec![GrammaticalForm::Negative],
            },
            ConjugatedWord {
                kanji_form: adverbial(adjective_kanji_form, category),
                kana_form: adverbial(adjective_kana_form, category),
                grammatical_forms: vec![GrammaticalForm::Adverbial],
            },
            ConjugatedWord {
                kanji_form: past_negative(adjective_kanji_form, category),
                kana_form: past_negative(adjective_kana_form, category),
                grammatical_forms: vec![GrammaticalForm::Past, GrammaticalForm::Negative],
            },
        ])
    } else {
        None
    }
}

fn negative(adjective: &str, category: &str) -> String {
    let (ending, suffix_len) = match adjective.chars().rev().nth(1).unwrap() {
        'い' if category == "adj-ix" => ("よくない", 2),
        _ => ("くない", 1),
    };
    let prefix_len = adjective.len() - suffix_len * 3;
    [&adjective[..prefix_len], ending].concat()
}

fn past(adjective: &str, category: &str) -> String {
    let (ending, suffix_len) = match adjective.chars().rev().nth(1).unwrap() {
        'い' if category == "adj-ix" => ("よかった", 2),
        _ => ("かった", 1),
    };
    let prefix_len = adjective.len() - suffix_len * 3;
    [&adjective[..prefix_len], ending].concat()
}

fn te_form(adjective: &str, category: &str) -> String {
    let (ending, suffix_len) = match adjective.chars().rev().nth(1).unwrap() {
        'い' if category == "adj-ix" => ("よくて", 2),
        _ => ("くて", 1),
    };
    let prefix_len = adjective.len() - suffix_len * 3;
    [&adjective[..prefix_len], ending].concat()
}

fn adverbial(adjective: &str, category: &str) -> String {
    let (ending, suffix_len) = match adjective.chars().rev().nth(1).unwrap() {
        'い' if category == "adj-ix" => ("よく", 2),
        _ => ("く", 1),
    };
    let prefix_len = adjective.len() - suffix_len * 3;
    [&adjective[..prefix_len], ending].concat()
}

fn past_negative(adjective: &str, category: &str) -> String {
    let (ending, suffix_len) = match adjective.chars().rev().nth(1).unwrap() {
        'い' if category == "adj-ix" => ("よくなかった", 2),
        _ => ("くなかった", 1),
    };
    let prefix_len = adjective.len() - suffix_len * 3;
    [&adjective[..prefix_len], ending].concat()
}

lazy_static! {
    static ref ADJECTIVE_TAGS: HashSet<String> = vec![
        String::from("adj-i"),
        String::from("adj-ix"),
        // String::from("adj-t"),
    ]
    .into_iter()
    .collect();
}
