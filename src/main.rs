use anyhow::Result;
use build_dictionaries::build_composite_dicts;

mod basic_dictionaries;
mod build_dictionaries;
mod composite_dictionaries;
mod dict_paths;
mod kana_utils;

fn main() -> Result<()> {
    build_composite_dicts()
}
