#![feature(hash_extract_if)]
use crate::load_dictionaries::load_vec_from_bin;
use anyhow::Result;
use build_dictionaries::build_composite_dicts;
use composite_dictionaries::*;
use dictionary_paths::DICTIONARY_ENTRIES;
use parse_example_sentences::parse_example_sentences_from_tsv;
use query::*;
use std::io::{self, Write};
use structopt::StructOpt;

mod adjective_conjugation_utils;
mod basic_dictionaries;
mod build_dictionaries;
mod composite_dictionaries;
mod dictionary_paths;
mod kana_utils;
mod load_dictionaries;
mod parse_example_sentences;
mod query;
mod sentence;
mod verb_conjugation_utils;

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
    // combined dict
    let entries: Vec<DictionaryEntry> = load_vec_from_bin(DICTIONARY_ENTRIES)?;

    // longest word is a name 42 characters long
    let dict: Dictionary = Dictionary::create(&entries);
    let sentences = parse_example_sentences_from_tsv("resources/sample_sentences.tsv", &dict)?;
    let defects: Vec<&str> = sentences
        .iter()
        .filter(|sentence| sentence.decomposition.is_none())
        .map(|sentence| sentence.raw.as_ref())
        .collect();
    println!("{}", defects.len());
    for defect in defects {
        println!("{:#?}", defect);
    }

    if opt.args.is_empty() {
        loop {
            let empty_str = &mut String::new();
            let query: String = read_input(empty_str)?.join(" ");
            for result in dict.query(&query) {
                println!("{}", result)
            }
        }
    } else {
        for arg in opt.args {
            let query = arg.as_str();
            for result in dict.query(query) {
                println!("{}", result)
            }
        }
    }
    Ok(())
}
