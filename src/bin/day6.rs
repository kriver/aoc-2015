use regex::{Regex};
use aoc_2015::{int_from_capture, load};

#[macro_use]
extern crate lazy_static;

type Lights = [[u32; 1000]; 1000];
type Act = dyn Fn(&Lights, usize, usize, &Action) -> u32;

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
                    x: int_from_capture(&caps, 2),
                    y: int_from_capture(&caps, 3),
                },
                to: Coord {
                    x: int_from_capture(&caps, 4),
                    y: int_from_capture(&caps, 5),
                },
            },
            None => unreachable!(line)
        }
    }
}

fn act_1(lights: &Lights, x: usize, y: usize, action: &Action) -> u32 {
    let prev = &lights[y][x];
    match action {
        Action::On => 1,
        Action::Off => 0,
        Action::Toggle => 1 - prev,
    }
}

fn act_2(lights: &Lights, x: usize, y: usize, action: &Action) -> u32 {
    let prev = &lights[y][x];
    match action {
        Action::On => prev + 1,
        Action::Off => if prev > &0 { prev - 1 } else { 0 },
        Action::Toggle => prev + 2,
    }
}

fn process_instruction(lights: &mut Lights, from: &Coord, to: &Coord, action: &Action, act: &Act) {
    assert!(from.y <= to.y, "{} <= {}", from.y, to.y);
    for y in from.y..(to.y + 1) {
        assert!(from.x <= to.x, "{} <= {}", from.x, to.x);
        for x in from.x..(to.x + 1) {
            let xu = x as usize;
            let yu = y as usize;
            lights[yu][xu] = act(lights, xu, yu, action);
        }
    }
}

fn count_lights(instructions: &Vec<Instruction>, act: &Act) -> u32 {
    let mut lights: Lights = [[0u32; 1000]; 1000];
    for instr in instructions.iter() {
        process_instruction(&mut lights, &instr.from, &instr.to, &instr.action, act);
    }
    lights.iter()
        .map(|row| row.iter().fold(0, |acc, x| acc + *x))
        .fold(0, |acc, x| acc + x)
}

fn parse(lines: &Vec<String>) -> Vec<Instruction> {
    lines.iter()
        .map(|s| Instruction::new(s))
        .collect()
}

fn main() {
    let instructions = parse(&load("data/day6.txt"));
    assert_eq!(count_lights(&instructions, &act_1), 543903);
    assert_eq!(count_lights(&instructions, &act_2), 14687245);
}
