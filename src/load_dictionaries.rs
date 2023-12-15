use anyhow::Result;
use serde::Deserialize;
use std::{error::Error, fmt, path::Path};

pub fn load_vec_from_bin<D: for<'a> Deserialize<'a>>(path: &str) -> Result<Vec<D>> {
    if Path::new(path).exists() {
        let encoded = std::fs::read(path).unwrap_or_else(|e| {
            panic!(
                "Failed to read binary file of encoded dictionary: {}, reason {}",
                path, e
            )
        });
        let decoded: Vec<D> = bincode::deserialize(&encoded).unwrap_or_else(|e| {
            panic!("Failed to decode binary dictionary: {}, reason {}", path, e)
        });
        Ok(decoded)
    } else {
        Err(NoBuildError.into())
    }
}

#[derive(Debug)]
struct NoBuildError;

impl Error for NoBuildError {}

impl fmt::Display for NoBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dictionaries have not been build yet.\nPlease run the program again with flag '-r' enabled")
    }
}
