use std::collections::HashMap;
use aoc_2015::load;

fn is_nice(s: &str) -> bool {
    let mut vowels = 0;
    let mut has_double = false;
    let mut has_prohibited = false;
    let mut prev: Option<char> = None;
    for c in s.chars() {
        if "aeiou".contains(c) {
            vowels = vowels + 1;
        }
        if prev.is_some() {
            if prev.unwrap() == c {
                has_double = true;
            }
            let prohibited = format!("{}{}", prev.unwrap(), c);
            if "ab" == prohibited || "cd" == prohibited || "pq" == prohibited || "xy" == prohibited {
                has_prohibited = true;
            }
        }
        prev = Some(c);
    }
    vowels >= 3 && has_double && !has_prohibited
}

fn is_really_nice(s: &str) -> bool {
    let mut pairs: HashMap<String, usize> = HashMap::new();
    let mut repeat_double = false;
    let mut repeat_skip = false;
    let mut prevprev: Option<char> = None;
    let mut prev: Option<char> = None;
    for (i, c) in s.chars().enumerate() {
        if prev.is_some() {
            let digram = format!("{}{}", prev.unwrap(), c);
            match pairs.get(&digram) {
                Some(offset) => repeat_double = offset + 2 <= i,
                None => {
                    pairs.insert(digram, i);
                    ()
                }
            }
        }
        if prevprev.filter(|p| *p == c).is_some() {
            repeat_skip = true;
        }
        prevprev = prev;
        prev = Some(c);
    }
    repeat_double && repeat_skip
}

fn count_nice(strings: &Vec<String>, is_nice: &dyn Fn(&str) -> bool) -> usize {
    strings.iter()
        .filter(|s| is_nice(s.as_str()))
        .count()
}

fn main() {
    assert_eq!(is_nice("ugknbfddgicrmopn"), true);
    assert_eq!(is_nice("aaa"), true);
    assert_eq!(is_nice("jchzalrnumimnmhp"), false);
    assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
    assert_eq!(is_nice("dvszwmarrgswjxmb"), false);

    let strings = load("data/day5.txt");
    assert_eq!(count_nice(&strings, &is_nice), 258);

    assert_eq!(is_really_nice("qjhvhtzxzqqjkmpb"), true);
    assert_eq!(is_really_nice("xxyxx"), true);
    assert_eq!(is_really_nice("uurcxstgmygtbstg"), false);
    assert_eq!(is_really_nice("ieodomkazucvgmuy"), false);

    assert_eq!(count_nice(&strings, &is_really_nice), 53);
}
