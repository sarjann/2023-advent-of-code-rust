use std::fs;

pub fn load(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read file")
}
