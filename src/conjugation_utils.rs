use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub enum GrammaticalForm {
    Causative,
    Conditional,
    Desire,
    Imperative,
    Negative,
    Passive,
    Past,
    Plain,
    Polite,
    Potential,
    Short,
    TeForm,
    Volitional,
    ProvisionalConditional,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerbConjugation {
    pub kanji_form: String,
    pub kana_form: String,
    pub grammatical_forms: Vec<GrammaticalForm>,
}

pub fn generate_all_verb_conjugations(
    verb_kanji_form: &str,
    verb_reading_form: &str,
    tags: Vec<&String>,
) -> Vec<VerbConjugation> {
    if !tags.iter().any(|s| s.starts_with('v')) || verb_kanji_form.is_empty() {
        Vec::new()
    } else if let Some(category) = tags.into_iter().find(|&tag| VERB_TAGS.contains(tag)) {
        let verb_kana_form = match verb_reading_form {
            "" => verb_kanji_form,
            _ => verb_reading_form,
        };
        vec![
            // plain forms
            VerbConjugation {
                kanji_form: causative(verb_kanji_form, category, false),
                kana_form: causative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Causative],
            },
            VerbConjugation {
                kanji_form: causative_passive(verb_kanji_form, category, false),
                kana_form: causative_passive(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Causative, GrammaticalForm::Passive],
            },
            VerbConjugation {
                kanji_form: causative_short(verb_kanji_form, category, false),
                kana_form: causative_short(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Causative, GrammaticalForm::Short],
            },
            VerbConjugation {
                kanji_form: conditional(verb_kanji_form, category, false),
                kana_form: conditional(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Conditional],
            },
            VerbConjugation {
                kanji_form: desire(verb_kanji_form, category, false),
                kana_form: desire(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Desire],
            },
            VerbConjugation {
                kanji_form: imperative(verb_kanji_form, category, false),
                kana_form: imperative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Imperative],
            },
            VerbConjugation {
                kanji_form: passive(verb_kanji_form, category, false),
                kana_form: passive(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Passive],
            },
            VerbConjugation {
                kanji_form: potential(verb_kanji_form, category, false),
                kana_form: potential(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Potential],
            },
            VerbConjugation {
                kanji_form: provisional_conditional(verb_kanji_form, category, false),
                kana_form: provisional_conditional(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::ProvisionalConditional],
            },
            VerbConjugation {
                kanji_form: volitional(verb_kanji_form, category, false),
                kana_form: volitional(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Volitional],
            },
            // Negative forms
            VerbConjugation {
                kanji_form: negative(verb_kanji_form, category, false),
                kana_form: negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Negative],
            },
            VerbConjugation {
                kanji_form: causative_negative(verb_kanji_form, category, false),
                kana_form: causative_negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Causative, GrammaticalForm::Negative],
            },
            VerbConjugation {
                kanji_form: causative_passive_negative(verb_kanji_form, category, false),
                kana_form: causative_passive_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Passive,
                    GrammaticalForm::Negative,
                ],
            },
            VerbConjugation {
                kanji_form: conditional_negative(verb_kanji_form, category, false),
                kana_form: conditional_negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Conditional, GrammaticalForm::Negative],
            },
            VerbConjugation {
                kanji_form: desire_negative(verb_kanji_form, category, false),
                kana_form: desire_negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Desire, GrammaticalForm::Negative],
            },
            VerbConjugation {
                kanji_form: imperative_negative(verb_kanji_form, category, false),
                kana_form: imperative_negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Imperative, GrammaticalForm::Negative],
            },
            VerbConjugation {
                kanji_form: passive_negative(verb_kanji_form, category, false),
                kana_form: passive_negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Passive, GrammaticalForm::Negative],
            },
            VerbConjugation {
                kanji_form: potential_negative(verb_kanji_form, category, false),
                kana_form: potential_negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Potential, GrammaticalForm::Negative],
            },
            // Past forms
            VerbConjugation {
                kanji_form: past(verb_kanji_form, category, false),
                kana_form: past(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Past],
            },
            VerbConjugation {
                kanji_form: causative_past(verb_kanji_form, category, false),
                kana_form: causative_past(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Causative, GrammaticalForm::Past],
            },
            VerbConjugation {
                kanji_form: causative_passive_past(verb_kanji_form, category, false),
                kana_form: causative_passive_past(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Passive,
                    GrammaticalForm::Past,
                ],
            },
            VerbConjugation {
                kanji_form: desire_past(verb_kanji_form, category, false),
                kana_form: desire_past(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Desire, GrammaticalForm::Past],
            },
            VerbConjugation {
                kanji_form: passive_past(verb_kanji_form, category, false),
                kana_form: passive_past(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Passive, GrammaticalForm::Past],
            },
            VerbConjugation {
                kanji_form: potential_past(verb_kanji_form, category, false),
                kana_form: potential_past(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Potential, GrammaticalForm::Past],
            },
            // Past negative forms
            VerbConjugation {
                kanji_form: past_negative(verb_kanji_form, category, false),
                kana_form: past_negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Past, GrammaticalForm::Negative],
            },
            VerbConjugation {
                kanji_form: causative_past_negative(verb_kanji_form, category, false),
                kana_form: causative_past_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Past,
                    GrammaticalForm::Negative,
                ],
            },
            VerbConjugation {
                kanji_form: causative_passive_past_negative(verb_kanji_form, category, false),
                kana_form: causative_passive_past_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Passive,
                    GrammaticalForm::Past,
                    GrammaticalForm::Negative,
                ],
            },
            VerbConjugation {
                kanji_form: desire_past_negative(verb_kanji_form, category, false),
                kana_form: desire_past_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Desire,
                    GrammaticalForm::Past,
                    GrammaticalForm::Negative,
                ],
            },
            VerbConjugation {
                kanji_form: passive_past_negative(verb_kanji_form, category, false),
                kana_form: passive_past_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Passive,
                    GrammaticalForm::Past,
                    GrammaticalForm::Negative,
                ],
            },
            VerbConjugation {
                kanji_form: potential_past_negative(verb_kanji_form, category, false),
                kana_form: potential_past_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Potential,
                    GrammaticalForm::Past,
                    GrammaticalForm::Negative,
                ],
            },
            // Te-forms
            VerbConjugation {
                kanji_form: te_form(verb_kanji_form, category, false),
                kana_form: te_form(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::TeForm],
            },
            VerbConjugation {
                kanji_form: causative_te(verb_kanji_form, category, false),
                kana_form: causative_te(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Causative, GrammaticalForm::TeForm],
            },
            VerbConjugation {
                kanji_form: causative_passive_te(verb_kanji_form, category, false),
                kana_form: causative_passive_te(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Passive,
                    GrammaticalForm::TeForm,
                ],
            },
            VerbConjugation {
                kanji_form: passive_te(verb_kanji_form, category, false),
                kana_form: passive_te(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Passive, GrammaticalForm::TeForm],
            },
            VerbConjugation {
                kanji_form: potential_te(verb_kanji_form, category, false),
                kana_form: potential_te(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Potential, GrammaticalForm::TeForm],
            },
            // Polite forms
            VerbConjugation {
                kanji_form: polite(verb_kanji_form, category, false),
                kana_form: polite(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Polite],
            },
            VerbConjugation {
                kanji_form: causative_polite(verb_kanji_form, category, false),
                kana_form: causative_polite(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Causative, GrammaticalForm::Polite],
            },
            VerbConjugation {
                kanji_form: causative_passive_polite(verb_kanji_form, category, false),
                kana_form: causative_passive_polite(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Passive,
                    GrammaticalForm::Polite,
                ],
            },
            VerbConjugation {
                kanji_form: passive_polite(verb_kanji_form, category, false),
                kana_form: passive_polite(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Passive, GrammaticalForm::Polite],
            },
            VerbConjugation {
                kanji_form: potential_polite(verb_kanji_form, category, false),
                kana_form: potential_polite(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Potential, GrammaticalForm::Polite],
            },
            VerbConjugation {
                kanji_form: volitional_polite(verb_kanji_form, category, false),
                kana_form: volitional_polite(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Volitional, GrammaticalForm::Polite],
            },
            // Polite negative forms
            VerbConjugation {
                kanji_form: polite_negative(verb_kanji_form, category, false),
                kana_form: polite_negative(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Polite, GrammaticalForm::Negative],
            },
            VerbConjugation {
                kanji_form: causative_polite_negative(verb_kanji_form, category, false),
                kana_form: causative_polite_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Polite,
                    GrammaticalForm::Negative,
                ],
            },
            VerbConjugation {
                kanji_form: causative_passive_polite_negative(verb_kanji_form, category, false),
                kana_form: causative_passive_polite_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Passive,
                    GrammaticalForm::Polite,
                    GrammaticalForm::Negative,
                ],
            },
            VerbConjugation {
                kanji_form: passive_polite_negative(verb_kanji_form, category, false),
                kana_form: passive_polite_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Passive,
                    GrammaticalForm::Polite,
                    GrammaticalForm::Negative,
                ],
            },
            VerbConjugation {
                kanji_form: potential_polite_negative(verb_kanji_form, category, false),
                kana_form: potential_polite_negative(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Potential,
                    GrammaticalForm::Polite,
                    GrammaticalForm::Negative,
                ],
            },
            // Polite past forms
            VerbConjugation {
                kanji_form: polite_past(verb_kanji_form, category, false),
                kana_form: polite_past(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Polite, GrammaticalForm::Past],
            },
            VerbConjugation {
                kanji_form: causative_polite_past(verb_kanji_form, category, false),
                kana_form: causative_polite_past(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Polite,
                    GrammaticalForm::Past,
                ],
            },
            VerbConjugation {
                kanji_form: causative_passive_polite_past(verb_kanji_form, category, false),
                kana_form: causative_passive_polite_past(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Causative,
                    GrammaticalForm::Passive,
                    GrammaticalForm::Polite,
                    GrammaticalForm::Past,
                ],
            },
            VerbConjugation {
                kanji_form: passive_polite_past(verb_kanji_form, category, false),
                kana_form: passive_polite_past(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Passive,
                    GrammaticalForm::Polite,
                    GrammaticalForm::Past,
                ],
            },
            VerbConjugation {
                kanji_form: potential_polite_past(verb_kanji_form, category, false),
                kana_form: potential_polite_past(verb_kana_form, category, true),
                grammatical_forms: vec![
                    GrammaticalForm::Potential,
                    GrammaticalForm::Polite,
                    GrammaticalForm::Past,
                ],
            },
            // Polite te-form
            VerbConjugation {
                kanji_form: polite_te_form(verb_kanji_form, category, false),
                kana_form: polite_te_form(verb_kana_form, category, true),
                grammatical_forms: vec![GrammaticalForm::Polite, GrammaticalForm::TeForm],
            },
        ]
    } else {
        Vec::new()
    }
}

fn masu_stem(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("", 1),
        "v5aru" | "v5u" | "v5u-s" => ("い", 1),
        "v5b" => ("び", 1),
        "v5g" => ("ぎ", 1),
        "v5k" | "v5k-s" => ("き", 1),
        "v5m" => ("み", 1),
        "v5n" => ("に", 1),
        "v5r" | "v5r-i" => ("り", 1),
        "v5s" => ("し", 1),
        "v5t" => ("ち", 1),
        "vk" => match reading {
            true => ("き", 2),
            false => ("", 1),
        },
        "vs-i" | "vs-s" => ("し", 2),
        "vz" => ("じ", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn causative(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "v5s" => ("させる", 1),
        "v5aru" | "v5r" | "v5r-i" => ("らせる", 1),
        "v5b" => ("ばせる", 1),
        "v5g" => ("がせる", 1),
        "v5k" | "v5k-s" => ("かせる", 1),
        "v5m" => ("ませる", 1),
        "v5n" => ("なせる", 1),
        "v5t" => ("たせる", 1),
        "v5u" | "v5u-s" => ("わせる", 1),
        "vk" => match reading {
            true => ("こさせる", 2),
            false => ("させる", 1),
        },
        "vs-i" | "vs-s" => ("させる", 2),
        "vz" => ("じさせる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn causative_passive(verb: &str, category: &str, reading: bool) -> String {
    let causative = causative(verb, category, reading);
    [&causative[..causative.len() - 3], "られる"].concat()
}

fn causative_short(verb: &str, category: &str, reading: bool) -> String {
    let causative = causative(verb, category, reading);
    [&causative[..causative.len() - 6], "す"].concat()
}

fn conditional(verb: &str, category: &str, reading: bool) -> String {
    past(verb, category, reading) + "ら"
}

fn desire(verb: &str, category: &str, reading: bool) -> String {
    masu_stem(verb, category, reading) + "たい"
}

fn imperative(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" => ("ろ", 1),
        "v1-s" => ("", 0),
        "v5aru" => ("い", 1),
        "v5b" => ("べ", 1),
        "v5g" => ("げ", 1),
        "v5k" | "v5k-s" => ("け", 1),
        "v5m" => ("め", 1),
        "v5n" => ("ね", 1),
        "v5r" | "v5r-i" => ("れ", 1),
        "v5s" => ("せ", 1),
        "v5t" => ("て", 1),
        "v5u" | "v5u-s" => ("え", 1),
        "vk" => match reading {
            true => ("こい", 2),
            false => ("い", 1),
        },
        "vs-i" | "vs-s" => ("しろ", 2),
        "vz" => ("じろ", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn passive(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "v5aru" | "v5r" | "v5r-i" => ("られる", 1),
        "v5b" => ("ばれる", 1),
        "v5g" => ("がれる", 1),
        "v5k" | "v5k-s" => ("かれる", 1),
        "v5m" => ("まれる", 1),
        "v5n" => ("なれる", 1),
        "v5s" => ("される", 1),
        "v5t" => ("たれる", 1),
        "v5u" | "v5u-s" => ("われる", 1),
        "vk" => match reading {
            true => ("きれる", 2),
            false => ("れる", 1),
        },
        "vs-i" | "vs-s" => ("される", 2),
        "vz" => ("じられる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn potential(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("られる", 1),
        "v5aru" => ("り得る", 1),
        "v5b" => ("べる", 1),
        "v5g" => ("げる", 1),
        "v5k" | "v5k-s" => ("ける", 1),
        "v5m" => ("める", 1),
        "v5n" => ("ねる", 1),
        "v5r" => ("れる", 1),
        "v5r-i" => ("りえる", 1),
        "v5s" => ("せる", 1),
        "v5t" => ("てる", 1),
        "v5u" | "v5u-s" => ("える", 1),
        "vk" => match reading {
            true => ("こられる", 2),
            false => ("られる", 1),
        },
        "vs-i" | "vs-s" => ("できる", 2),
        "vz" => ("じられる", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn provisional_conditional(verb: &str, category: &str, _reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" | "v5aru" | "v5r" | "v5r-i" | "vs-i" | "vs-s" | "vz" | "vk" => ("れば", 1),
        "v5b" => ("べば", 1),
        "v5g" => ("げば", 1),
        "v5k" | "v5k-s" => ("けば", 1),
        "v5m" => ("めば", 1),
        "v5n" => ("ねば", 1),
        "v5s" => ("せば", 1),
        "v5t" => ("てば", 1),
        "v5u" | "v5u-s" => ("えば", 1),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn volitional(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("よう", 1),
        "v5aru" | "v5r" | "v5r-i" => ("ろう", 1),
        "v5b" => ("ぼう", 1),
        "v5g" => ("ごう", 1),
        "v5k" | "v5k-s" => ("こう", 1),
        "v5m" => ("もう", 1),
        "v5n" => ("のう", 1),
        "v5s" => ("そう", 1),
        "v5t" => ("とう", 1),
        "v5u" | "v5u-s" => ("おう", 1),
        "vk" => match reading {
            true => ("こよう", 2),
            false => ("よう", 1),
        },
        "vs-i" | "vs-s" => ("しよう", 2),
        "vz" => ("じよう", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn negative(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("ない", 1),
        "v5b" => ("ばない", 1),
        "v5g" => ("がない", 1),
        "v5k" | "v5k-s" => ("かない", 1),
        "v5m" => ("まない", 1),
        "v5n" => ("なない", 1),
        "v5r" | "v5aru" => ("らない", 1),
        "v5r-i" => ("ない", 2),
        "v5s" => ("さない", 1),
        "v5t" => ("たない", 1),
        "v5u" | "v5u-s" => ("わない", 1),
        "vk" => match reading {
            true => ("こない", 2),
            false => ("ない", 1),
        },
        "vs-i" | "vs-s" => ("しない", 2),
        "vz" => ("じない", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn causative_negative(verb: &str, category: &str, reading: bool) -> String {
    negative(&causative(verb, category, reading), "v1", reading)
}

fn causative_passive_negative(verb: &str, category: &str, reading: bool) -> String {
    negative(&causative(verb, category, reading), "v1", reading)
}

fn conditional_negative(verb: &str, category: &str, reading: bool) -> String {
    past_negative(verb, category, reading) + "ら"
}

fn desire_negative(verb: &str, category: &str, reading: bool) -> String {
    let tai = desire(verb, category, reading);
    [&tai[..tai.len() - 3], "くない"].concat()
}

fn imperative_negative(verb: &str, category: &str, reading: bool) -> String {
    imperative(verb, category, reading) + "な"
}

fn passive_negative(verb: &str, category: &str, reading: bool) -> String {
    negative(&passive(verb, category, reading), "v1", reading)
}

fn potential_negative(verb: &str, category: &str, reading: bool) -> String {
    negative(&potential(verb, category, reading), "v1", reading)
}

fn past(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("た", 1),
        "v5aru" | "v5k-s" | "v5r" | "v5r-i" | "v5t" | "v5u" => ("った", 1),
        "v5b" | "v5m" | "v5n" => ("んだ", 1),
        "v5g" => ("いだ", 1),
        "v5k" => ("いた", 1),
        "v5s" => ("した", 1),
        "v5u-s" => ("うた", 1),
        "vk" => match reading {
            true => ("きた", 2),
            false => ("た", 1),
        },
        "vs-i" | "vs-s" => ("した", 2),
        "vz" => ("じた", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn causative_past(verb: &str, category: &str, reading: bool) -> String {
    past(&causative(verb, category, reading), "v1", reading)
}

fn causative_passive_past(verb: &str, category: &str, reading: bool) -> String {
    past(&causative_passive(verb, category, reading), "v1", reading)
}

fn desire_past(verb: &str, category: &str, reading: bool) -> String {
    let tai = desire(verb, category, reading);
    [&tai[..tai.len() - 3], "かった"].concat()
}

fn passive_past(verb: &str, category: &str, reading: bool) -> String {
    past(&passive(verb, category, reading), "v1", reading)
}

fn potential_past(verb: &str, category: &str, reading: bool) -> String {
    past(&potential(verb, category, reading), "v1", reading)
}

fn past_negative(verb: &str, category: &str, reading: bool) -> String {
    let negative = negative(verb, category, reading);
    [&negative[..negative.len() - 3], "かった"].concat()
}

fn causative_past_negative(verb: &str, category: &str, reading: bool) -> String {
    past_negative(&causative(verb, category, reading), "v1", reading)
}

fn causative_passive_past_negative(verb: &str, category: &str, reading: bool) -> String {
    past_negative(&causative_passive(verb, category, reading), "v1", reading)
}

fn desire_past_negative(verb: &str, category: &str, reading: bool) -> String {
    let tai = desire(verb, category, reading);
    [&tai[..tai.len() - 3], "くなかった"].concat()
}

fn passive_past_negative(verb: &str, category: &str, reading: bool) -> String {
    past_negative(&passive(verb, category, reading), "v1", reading)
}

fn potential_past_negative(verb: &str, category: &str, reading: bool) -> String {
    past_negative(&potential(verb, category, reading), "v1", reading)
}

fn te_form(verb: &str, category: &str, reading: bool) -> String {
    let (ending, suffix_len) = match category {
        "v1" | "v1-s" => ("て", 1),
        "v5aru" | "v5k-s" | "v5r" | "v5r-i" | "v5t" | "v5u" => ("って", 1),
        "v5b" | "v5m" | "v5n" => ("んで", 1),
        "v5g" => ("いで", 1),
        "v5k" => ("いて", 1),
        "v5s" => ("して", 1),
        "v5u-s" => ("うて", 1),
        "vk" => match reading {
            true => ("きて", 2),
            false => ("て", 1),
        },
        "vs-i" | "vs-s" => ("して", 2),
        "vz" => ("じて", 2),
        _ => panic!("format '{}' unknown", category),
    };
    let prefix_len = verb.len() - suffix_len * 3;
    [&verb[..prefix_len], ending].concat()
}

fn causative_te(verb: &str, category: &str, reading: bool) -> String {
    te_form(&causative(verb, category, reading), "v1", reading)
}

fn causative_passive_te(verb: &str, category: &str, reading: bool) -> String {
    te_form(&causative_passive(verb, category, reading), "v1", reading)
}

fn passive_te(verb: &str, category: &str, reading: bool) -> String {
    te_form(&passive(verb, category, reading), "v1", reading)
}

fn potential_te(verb: &str, category: &str, reading: bool) -> String {
    te_form(&potential(verb, category, reading), "v1", reading)
}

fn polite(verb: &str, category: &str, reading: bool) -> String {
    masu_stem(verb, category, reading) + "ます"
}

fn causative_polite(verb: &str, category: &str, reading: bool) -> String {
    let causative = causative(verb, category, reading);
    [&causative[..causative.len() - 3], "ます"].concat()
}

fn causative_passive_polite(verb: &str, category: &str, reading: bool) -> String {
    let causative_passive = causative_passive(verb, category, reading);
    [&causative_passive[..causative_passive.len() - 3], "ます"].concat()
}

fn passive_polite(verb: &str, category: &str, reading: bool) -> String {
    let passive = passive(verb, category, reading);
    [&passive[..passive.len() - 3], "ます"].concat()
}

fn potential_polite(verb: &str, category: &str, reading: bool) -> String {
    let potential = potential(verb, category, reading);
    [&potential[..potential.len() - 3], "ます"].concat()
}

fn volitional_polite(verb: &str, category: &str, reading: bool) -> String {
    masu_stem(verb, category, reading) + "ましょう"
}

fn polite_negative(verb: &str, category: &str, reading: bool) -> String {
    masu_stem(verb, category, reading) + "ません"
}

fn causative_polite_negative(verb: &str, category: &str, reading: bool) -> String {
    let causative = causative(verb, category, reading);
    [&causative[..causative.len() - 3], "ません"].concat()
}

fn causative_passive_polite_negative(verb: &str, category: &str, reading: bool) -> String {
    let causative_passive = causative_passive(verb, category, reading);
    [&causative_passive[..causative_passive.len() - 3], "ません"].concat()
}

fn passive_polite_negative(verb: &str, category: &str, reading: bool) -> String {
    let passive = passive(verb, category, reading);
    [&passive[..passive.len() - 3], "ません"].concat()
}

fn potential_polite_negative(verb: &str, category: &str, reading: bool) -> String {
    let potential = potential(verb, category, reading);
    [&potential[..potential.len() - 3], "ません"].concat()
}

fn polite_past(verb: &str, category: &str, reading: bool) -> String {
    masu_stem(verb, category, reading) + "ました"
}

fn causative_polite_past(verb: &str, category: &str, reading: bool) -> String {
    let causative = causative(verb, category, reading);
    [&causative[..causative.len() - 3], "ました"].concat()
}

fn causative_passive_polite_past(verb: &str, category: &str, reading: bool) -> String {
    let causative_passive = causative_passive(verb, category, reading);
    [&causative_passive[..causative_passive.len() - 3], "ました"].concat()
}

fn passive_polite_past(verb: &str, category: &str, reading: bool) -> String {
    let passive = passive(verb, category, reading);
    [&passive[..passive.len() - 3], "ました"].concat()
}

fn potential_polite_past(verb: &str, category: &str, reading: bool) -> String {
    let potential = potential(verb, category, reading);
    [&potential[..potential.len() - 3], "ました"].concat()
}

fn polite_te_form(verb: &str, category: &str, reading: bool) -> String {
    masu_stem(verb, category, reading) + "まして"
}

lazy_static! {
    static ref VERB_TAGS: HashSet<String> = vec![
        String::from("v1"),
        String::from("v1-s"),
        String::from("vk"),
        String::from("v5b"),
        String::from("v5g"),
        String::from("v5k"),
        String::from("v5k-s"),
        String::from("v5m"),
        String::from("v5n"),
        String::from("v5r"),
        String::from("v5aru"),
        String::from("v5r-i"),
        String::from("v5s"),
        String::from("v5t"),
        String::from("v5u"),
        String::from("v5u-s"),
        String::from("vs-i"),
        String::from("vs-s"),
        String::from("vz"),
    ]
    .into_iter()
    .collect();
}
