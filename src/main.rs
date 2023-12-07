use build_dictionaries::build_composite_dicts;

mod build_dictionaries;
mod composite_dictionaries;
mod dict_paths;
mod innocent_dictionary;
mod jmdict_dictionary;
mod kana_utils;
mod kanji_dictionaries;
mod radical_dictionaries;
mod tag;

fn main() {
    build_composite_dicts();
}
