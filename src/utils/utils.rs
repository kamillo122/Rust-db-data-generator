use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open names file");
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}
