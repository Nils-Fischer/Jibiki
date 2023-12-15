#![feature(hash_extract_if)]
use crate::load_dictionaries::load_vec_from_bin;
use anyhow::Result;
use build_dictionaries::{build_composite_dicts, hashmap_of_dicts};
use composite_dictionaries::*;
use dictionary_paths::{
    KANJIS_EXPORT_PATH, NAMES_EXPORT_PATH, RADICALS_EXPORT_PATH, WORDS_EXPORT_PATH,
};
use query::to_queriable_dict;
use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
};
use structopt::StructOpt;

mod basic_dictionaries;
mod build_dictionaries;
mod composite_dictionaries;
mod dictionary_paths;
mod kana_utils;
mod load_dictionaries;
mod query;

#[derive(StructOpt)]
#[structopt(
    name = "Kanji Dictionary",
    about = "A Command Line Kanji Dictionary",
    author = "Nils Fischer"
)]
struct Opt {
    #[structopt(short, long)]
    rebuild: bool,

    args: Vec<String>,
}

fn read_input() -> Result<Vec<String>> {
    print!("Enter query: ");

    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    Ok(input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.trim().to_string())
        .collect())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    if opt.rebuild {
        match build_composite_dicts() {
            Ok(_) => println!("All binaries were successfully rebuild!\n"),
            Err(_) => println!("Failed to rebuild all binaries!\n"),
        }
    }

    let words: Vec<Word> = load_vec_from_bin(WORDS_EXPORT_PATH)?;
    let kanjis: Vec<Kanji> = load_vec_from_bin(KANJIS_EXPORT_PATH)?;
    let names: Vec<Name> = load_vec_from_bin(NAMES_EXPORT_PATH)?;
    let radicals: Vec<Radical> = load_vec_from_bin(RADICALS_EXPORT_PATH)?;

    let word_dict: HashMap<&String, Vec<&Word>> = to_queriable_dict(&words);
    let kanji_dict: HashMap<&String, Vec<&Kanji>> = to_queriable_dict(&kanjis);
    let name_dict: HashMap<&String, Vec<&Name>> = to_queriable_dict(&names);
    let radical_dict: HashMap<&String, Vec<&Radical>> = to_queriable_dict(&radicals);

    if opt.args.is_empty() {
        loop {
            let queries: Vec<String> = read_input()?;
            for query in queries {
                if let Some(result) = word_dict.get(&query) {
                    println!("{:#?}", result)
                }
                if let Some(result) = kanji_dict.get(&query) {
                    println!("{:#?}", result)
                }
                if let Some(result) = name_dict.get(&query) {
                    println!("{:#?}", result)
                }
                if let Some(result) = radical_dict.get(&query) {
                    println!("{:#?}", result)
                }
            }
        }
    } else {
        for query in opt.args {
            if let Some(result) = word_dict.get(&query) {
                println!("{:#?}", result)
            }
            if let Some(result) = kanji_dict.get(&query) {
                println!("{:#?}", result)
            }
            if let Some(result) = name_dict.get(&query) {
                println!("{:#?}", result)
            }
            if let Some(result) = radical_dict.get(&query) {
                println!("{:#?}", result)
            }
        }
    }
    Ok(())
}
