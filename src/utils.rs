use itertools::Itertools;
use std::fs;

/// takes a path for the location of the text file and converts to a Vec<str>.
pub fn convert_file_to_vec(path: String) -> Vec<String> {
    let file: String = fs::read_to_string(path).expect("Should have been able to read the file");
    return file
        .split("\n")
        .map(|el: &str| el.to_string())
        .collect_vec();
}
