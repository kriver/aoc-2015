use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Captures;

pub fn load(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
        .map(|l| l.unwrap())
        .collect()
}

pub fn str_from_capture<'a>(captures: &'a Captures, i: usize) -> String {
    match captures.get(i) {
        Some(m) => m.as_str().to_string(),
        None => unreachable!("expected string for capture {}", i)
    }
}

pub fn int_from_capture(captures: &Captures, i: usize) -> u32 {
    match captures.get(i) {
        Some(m) => match m.as_str().parse() {
            Ok(ival) => ival,
            Err(_) => unreachable!("expected int for capture {}: {}", i, m.as_str())
        },
        None => unreachable!("expected int for capture {}", i)
    }
}
