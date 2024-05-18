use std::fmt;

use crate::DictionaryEntry;

#[derive(Debug)]
pub struct Sentence<'a> {
    raw: String,
    decomposition: Option<Vec<ParsedWord<'a>>>,
    id: u32,
}

impl<'a> Sentence<'a> {
    pub fn create(raw: String, decomposition: Option<Vec<ParsedWord<'a>>>, id: u32) -> Self {
        Sentence {
            raw,
            decomposition,
            id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedWord<'a> {
    word: String,
    interpretations: Vec<&'a DictionaryEntry>,
}

impl<'a> ParsedWord<'a> {
    pub fn create(word: String, interpretations: Vec<&'a DictionaryEntry>) -> Self {
        ParsedWord {
            word,
            interpretations,
        }
    }
}

impl<'a> fmt::Display for ParsedWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Word: {}", self.word)?;
        writeln!(f, "----------------------------------------")?;
        for interpretation in self.interpretations.iter() {
            writeln!(f, "{}", interpretation)?;
        }
        writeln!(f, "----------------------------------------")?;
        Ok(())
    }
}
