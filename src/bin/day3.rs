use std::collections::HashSet;
use phf::phf_map;
use aoc_2015::load;

static DIR: phf::Map<char, (i32, i32)> = phf_map! {
    '<' => (-1, 0),
    '>' => (1, 0),
    '^' => (0, -1),
    'v' => (0, 1)
};

fn count_houses(directions: &str, num_santas: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut positions: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];
    visited.insert((0, 0));
    // travel
    let mut santa = 0;
    for c in directions.chars() {
        let (dx, dy) = DIR.get(&c).expect("valid direction");
        positions[santa].0 = positions[santa].0 + *dx;
        positions[santa].1 = positions[santa].1 + *dy;
        visited.insert((positions[santa].0, positions[santa].1));
        santa = (santa + 1) % num_santas;
    }
    visited.len()
}

fn main() {
    assert_eq!(count_houses(">", 1), 2);
    assert_eq!(count_houses("^>v<", 1), 4);
    assert_eq!(count_houses("^v^v^v^v^v", 1), 2);

    let directions = load("data/day3.txt");
    assert_eq!(count_houses(directions[0].as_str(), 1), 2572);

    assert_eq!(count_houses("^v", 2), 3);
    assert_eq!(count_houses("^>v<", 2), 3);
    assert_eq!(count_houses("^v^v^v^v^v", 2), 11);
    assert_eq!(count_houses(directions[0].as_str(), 2), 2631);
}
