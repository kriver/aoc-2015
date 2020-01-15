use regex::{Regex};
use aoc_2015::{str_from_capture, int_from_capture, load};
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;

type Locations = HashSet<String>;
type Distances = HashMap<String, HashMap<String, u32>>;
type Cmp = dyn Fn(u32, u32) -> bool;

fn parse(lines: &[String]) -> (Locations, Distances) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-zA-Z]+) to ([a-zA-Z]+) = (\d+)").unwrap();
    }
    let mut stars = HashSet::new();
    let mut dist = HashMap::new();
    for line in lines {
        match RE.captures(line) {
            Some(caps) => {
                let star1 = str_from_capture(&caps, 1);
                let star2 = str_from_capture(&caps, 2);
                let dval = int_from_capture(&caps, 3);
                stars.insert(star1.clone());
                stars.insert(star2.clone());
                if !dist.contains_key(&star1) {
                    dist.insert(star1.clone(), HashMap::new());
                }
                if !dist.contains_key(&star2) {
                    dist.insert(star2.clone(), HashMap::new());
                }
                dist.get_mut(&star1).unwrap().insert(star2.clone(), dval);
                dist.get_mut(&star2).unwrap().insert(star1.clone(), dval);
            },
            None => unreachable!(line)
        }
    }
    (stars, dist)
}

fn tsp_starting_from(start: &str, locations: &Locations, distances: &Distances,
                     cmp: &Cmp, init: u32) -> u32 {
    let mut total_dist = 0;
    let mut path: Vec<&str> = Vec::new();
    let mut current: &str = start;
    loop {
        path.push(current);
        if path.len() == locations.len() {
            break;
        }
        let mut dist: u32 = init;
        let mut next: Option<&String> = None;
        for (name, d) in &distances[current] {
            if !path.contains(&name.as_str()) && cmp(*d, dist) {
                dist = *d;
                next = Some(name);
            }
        }
        match next {
            Some(name) => {
                current = name;
                total_dist += dist;
            },
            None => panic!("no star found")
        };
    }
    total_dist
}

fn tsp(locations: &Locations, distances: &Distances, cmp: &Cmp, init: u32) -> u32 {
    let mut distance = init;
    for loc in locations {
        let d = tsp_starting_from(loc.as_str(), &locations, &distances, cmp, init);
        if cmp(d, distance) {
            distance = d;
        }
    }
    distance
}

fn main() {
    let data = load("data/day9.txt");
    let (locations, distances) = parse(&data);

    assert_eq!(tsp(&locations, &distances, &|a, b| a < b, std::u32::MAX), 141);
    assert_eq!(tsp(&locations, &distances, &|a, b| a > b, 0), 736);
}
