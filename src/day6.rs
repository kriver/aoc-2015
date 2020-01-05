use util::load;
use std::collections::HashSet;
use regex::{Regex, Match};

#[macro_use]
extern crate lazy_static;

type Lights = HashSet<(u32, u32)>;

enum Action {
    On,
    Off,
    Toggle,
}

struct Coord {
    x: u32,
    y: u32,
}

struct Instruction {
    action: Action,
    from: Coord,
    to: Coord,
}

impl Action {
    pub fn new(s: &str) -> Action {
        match s {
            "on" => Action::On,
            "off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => unreachable!(s)
        }
    }
}

fn match_to_integer(m: &Option<Match>) -> u32 {
    match m.map(|x| x.as_str()) {
        Some(s) => match s.parse() {
            Ok(i) => i,
            Err(_) => unreachable!(s)
        },
        None => unreachable!("should be integer string")
    }
}

impl Instruction {
    pub fn new(line: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?:turn )?([a-z]+) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        }
        match RE.captures(line) {
            Some(caps) => Instruction {
                action: match caps.get(1) {
                    Some(m) => Action::new(m.as_str()),
                    None => unreachable!("should be action string")
                },
                from: Coord {
                    x: match_to_integer(&caps.get(2)),
                    y: match_to_integer(&caps.get(3)),
                },
                to: Coord {
                    x: match_to_integer(&caps.get(4)),
                    y: match_to_integer(&caps.get(5)),
                },
            },
            None => unreachable!(line)
        }
    }
}

fn process_instruction(lights: &mut Lights, from: &Coord, to: &Coord, action: &Action) {
    for y in from.y..(to.y + 1) {
        for x in from.x..(to.x + 1) {
            match action {
                Action::On => { lights.insert((x, y)); },
                Action::Off => { lights.remove(&(x, y)); },
                Action::Toggle =>
                    if lights.contains(&(x, y)) {
                        lights.remove(&(x, y));
                    } else {
                        lights.insert((x, y));
                    }
            }
        }
    }
}

fn count_lights(instructions: &Vec<Instruction>) -> usize {
    let mut lights: Lights = HashSet::new();
    for (i, instr) in instructions.iter().enumerate() {
        println!("Processing step {}", i);
        process_instruction(&mut lights, &instr.from, &instr.to, &instr.action);
    }
    lights.len()
}

fn parse(lines: &Vec<String>) -> Vec<Instruction> {
    lines.iter()
        .map(|s| Instruction::new(s))
        .collect()
}

fn main() {
    let instructions = parse(&load("data/day6.txt"));
    assert_eq!(count_lights(&instructions), 543903);
}
