use std::fs::File;
use std::io::{BufReader, BufRead};

fn load(filename: &str) -> String {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("successful read");
    return buf;
}

fn floor(directions: &str) -> (i32, Option<i32>) {
    let mut floor = 0;
    let mut pos = 0;
    let mut basement_found = false;
    for c in directions.chars() {
        if !basement_found {
            pos = pos + 1;
        }
        floor = if c == '(' { floor + 1 } else { floor - 1 };
        if floor < 0 {
            basement_found = true;
        }
    }
    (floor, if basement_found { Some(pos) } else { None })
}

fn main() {
    assert_eq!(floor("(())"), (0, None));
    assert_eq!(floor("()()"), (0, None));
    assert_eq!(floor("((("), (3, None));
    assert_eq!(floor("(()(()("), (3, None));
    assert_eq!(floor("))((((("), (3, Some(1)));
    assert_eq!(floor("())"), (-1, Some(3)));
    assert_eq!(floor("))("), (-1, Some(1)));
    assert_eq!(floor(")))"), (-3, Some(1)));
    assert_eq!(floor(")())())"), (-3, Some(1)));

    let directions = load("data/day1.txt");
    assert_eq!(floor(directions.as_str()), (138, Some(1771)));
}
