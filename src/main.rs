use crate::load_dictionaries::load_vec_from_bin;
use anyhow::Result;
use build_dictionaries::build_composite_dicts;
use composite_dictionaries::*;
use dict_paths::{KANJIS_EXPORT_PATH, NAMES_EXPORT_PATH, RADICALS_EXPORT_PATH, WORDS_EXPORT_PATH};
use std::io::{self, BufRead};
use structopt::StructOpt;

mod basic_dictionaries;
mod build_dictionaries;
mod composite_dictionaries;
mod dict_paths;
mod kana_utils;
mod load_dictionaries;

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

    let stdin = io::stdin();
    let inputs: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(|line| {
            line.ok().map(|line| {
                line.split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
        })
        .flatten()
        .collect();
    Ok(inputs)
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    if opt.rebuild {
        match build_composite_dicts() {
            Ok(_) => println!("All binaries were successfully rebuild!\n"),
            Err(_) => println!("Failed to rebuild all binaries!\n"),
        }
    }

    let words: Vec<Word> = load_vec_from_bin(&WORDS_EXPORT_PATH)?;
    let kanjis: Vec<Kanji> = load_vec_from_bin(&KANJIS_EXPORT_PATH)?;
    let names: Vec<Name> = load_vec_from_bin(&NAMES_EXPORT_PATH)?;
    let radicals: Vec<Radical> = load_vec_from_bin(&RADICALS_EXPORT_PATH)?;

    Ok(())
}
