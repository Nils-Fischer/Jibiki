use anyhow::Result;
use build_dictionaries::build_composite_dicts;

mod basic_dictionaries;
mod build_dictionaries;
mod composite_dictionaries;
mod dict_paths;
mod innocent_dictionary;
mod jmdict_dictionary;
mod kana_utils;
mod kanji_dictionaries;
mod radical_dictionaries;
mod tag;

fn main() -> Result<()> {
    build_composite_dicts()
}
