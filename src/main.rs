#![feature(hash_extract_if)]
use crate::load_dictionaries::load_vec_from_bin;
use anyhow::Result;
use build_dictionaries::build_composite_dicts;
use composite_dictionaries::*;
use dictionary_paths::{
    KANJIS_EXPORT_PATH, NAMES_EXPORT_PATH, RADICALS_EXPORT_PATH, WORDS_EXPORT_PATH,
};
use query::*;
use std::io::{self, Write};
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

fn read_input(input: &mut String) -> Result<Vec<&str>> {
    print!("Enter query: ");

    io::stdout().flush()?;

    io::stdin().read_line(input).expect("Failed to read line");
    Ok(input.split_ascii_whitespace().collect())
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

    let dict = QueriableDict::new(&words, &kanjis, &names, &radicals);

    if opt.args.is_empty() {
        loop {
            let empty_str = &mut String::new();
            let queries: Vec<&str> = read_input(empty_str)?;
            for query in queries {
                dict.query(query)
            }
        }
    } else {
        for arg in opt.args {
            let query = arg.as_str();
            dict.query(query)
        }
    }
    Ok(())
}
