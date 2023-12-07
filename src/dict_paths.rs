pub fn jmdict_tag_paths() -> Vec<String> {
    vec!["resources/jmdict_english/tag_bank_1.json".to_string()]
}

pub fn jmdict_dict_paths() -> Vec<String> {
    (1..=32)
        .map(|i| format!("resources/jmdict_english/term_bank_{i}.json"))
        .collect()
}

pub fn jmnedict_tag_paths() -> Vec<String> {
    vec!["resources/jmnedict/tag_bank_1.json".to_string()]
}

pub fn jmnedict_dict_paths() -> Vec<String> {
    (1..=75)
        .map(|i| format!("resources/jmnedict/term_bank_{i}.json"))
        .collect()
}
pub fn kanjidic_tag_paths() -> Vec<String> {
    vec!["resources/kanjidic_english/tag_bank_1.json".to_string()]
}

pub fn kanjidic_dict_paths() -> Vec<String> {
    (1..=2)
        .map(|i| format!("resources/kanjidic_english/kanji_bank_{i}.json"))
        .collect()
}

pub fn kanjium_tag_paths() -> Vec<String> {
    vec!["resources/kanjium_pitch_accents/tag_bank_1.json".to_string()]
}

pub fn kanjium_dict_paths() -> Vec<String> {
    (1..=13)
        .map(|i| format!("resources/kanjium_pitch_accents/term_meta_bank_{i}.json"))
        .collect()
}

pub fn innocent_kanji_dict_paths() -> Vec<String> {
    vec!["resources/innocent_corpus/kanji_meta_bank_1.json".to_string()]
}

pub fn innocent_vocab_dict_paths() -> Vec<String> {
    (1..=29)
        .map(|i| format!("resources/innocent_corpus/term_meta_bank_{i}.json"))
        .collect()
}

pub fn krad_dict_paths() -> Vec<String> {
    vec!["resources/radicals/kradfile.json".to_string()]
}

pub fn radk_dict_paths() -> Vec<String> {
    vec!["resources/radicals/radkfilex.json".to_string()]
}

pub trait ExportPath {
    fn export_path(&self) -> String;
}
