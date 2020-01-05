use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn load(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|l| l.unwrap())
        .collect()
}
