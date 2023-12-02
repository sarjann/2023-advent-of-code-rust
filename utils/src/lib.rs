use std::fs;

pub fn load(path: &str) -> String {
    let new_path = format!("src/{}", path);
    fs::read_to_string(new_path).expect("Failed to read file")
}
