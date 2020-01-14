use aoc_2015::load;

fn count_decoded(s: &str) -> (usize, usize) {
    let mut sz = 0;
    let mut backslash = false;
    let mut skip = 0;
    // skip enclosing double quotes
    for c in s[1..s.len() - 1].chars() {
        if skip > 0 {
            skip -= 1
        } else if backslash {
            if c == 'x' {
                skip = 2
            }
            sz += 1; // count backslash'ed character
            backslash = false;
        } else if c == '\\' {
            backslash = true;
        } else {
            sz +=  1
        }
    }
    (s.len(), sz)
}

fn count_encoded(s: &str) -> (usize, usize) {
    let mut sz = 0;
    for c in s.chars() {
        match c {
            '\\' | '"' => sz += 2,
            _ => sz += 1
        }
    }
    (s.len(), 2 + sz) // extra 2 for quotes
}

fn count_all(lines: &[String], count: &dyn Fn(&str) -> (usize, usize)) -> (usize, usize) {
    lines.iter()
        .map(|s| count(s.as_str()))
        .fold((0, 0), |acc, sz| (acc.0 + sz.0, acc.1 + sz.1))
}

fn main() {
    let data = load("data/day8.txt");

    assert_eq!(count_decoded(r#""""#), (2, 0));
    assert_eq!(count_decoded(r#""abc""#), (5, 3));
    assert_eq!(count_decoded(r#""aaa\"aaa""#), (10, 7));
    assert_eq!(count_decoded(r#""\x27""#), (6, 1));
    let counts = count_all(&data, &count_decoded);
    assert_eq!(counts.0 - counts.1, 1350);

    assert_eq!(count_encoded(r#""""#), (2, 6));
    assert_eq!(count_encoded(r#""abc""#), (5, 9));
    assert_eq!(count_encoded(r#""aaa\"aaa""#), (10, 16));
    assert_eq!(count_encoded(r#""\x27""#), (6, 11));
    let counts = count_all(&data, &count_encoded);
    assert_eq!(counts.1 - counts.0, 2085);
}
