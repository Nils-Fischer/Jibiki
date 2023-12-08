use crate::{
    innocent_dictionary::Innocent,
    jmdict_dictionary::Jmdict,
    kanji_dictionaries::{Kanjidic, Kanjium},
    radical_dictionaries::{Krad, Radk},
};

#[allow(dead_code)]
pub enum BasicDicts {
    Jmdicts(Vec<Jmdict>),
    Jmnedicts(Vec<Jmdict>),
    Kanjiums(Vec<Kanjium>),
    Kanjidics(Vec<Kanjidic>),
    InnocentKanjis(Vec<Innocent>),
    InnocentVocabs(Vec<Innocent>),
    Krads(Vec<Krad>),
    Radks(Vec<Radk>),
}
