//! Data helpers

use std::fs;
pub fn file_to_vec_string(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect()
}
