use anyhow::Result;
use serde::Deserialize;

pub fn load_vec_from_bin<D: for<'a> Deserialize<'a>>(path: &str) -> Result<Vec<D>> {
    let encoded = std::fs::read(path).unwrap_or_else(|e| {
        panic!(
            "Failed to read binary file of encoded dictionary: {}, reason {}",
            path, e
        )
    });
    let decoded: Vec<D> = bincode::deserialize(&encoded)
        .unwrap_or_else(|e| panic!("Failed to decode binary dictionary: {}, reason {}", path, e));
    Ok(decoded)
}
