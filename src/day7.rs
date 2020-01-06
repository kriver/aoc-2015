use util::load;
use std::collections::HashMap;

type Wires<'a> = HashMap<&'a str, u16>;

#[derive(Debug)]
enum Gate {
    Assign,
    And,
    Or,
    Not,
    LShift,
    RShift,
}

impl Gate {
    pub fn new(s: &str) -> Gate {
        match s {
            "" => Gate::Assign,
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "NOT" => Gate::Not,
            "LSHIFT" => Gate::LShift,
            "RSHIFT" => Gate::RShift,
            _ => unreachable!(s)
        }
    }
}

#[derive(Debug)]
struct WireValue<'a> {
    wire: Option<&'a str>,
    value: Option<u16>,
}

impl WireValue<'_> {
    pub fn new(s: &str) -> WireValue {
        match s.parse() {
            Ok(i) => WireValue::new_value(i),
            Err(_) => WireValue::new_wire(s)
        }
    }

    pub fn new_value<'a>(v: u16) -> WireValue<'a> {
        WireValue { wire: None, value: Some(v) }
    }

    pub fn new_wire(w: &str) -> WireValue {
        WireValue { wire: Some(w), value: None }
    }

    pub fn apply<'a>(wv: WireValue<'a>, wires: &Wires) -> WireValue<'a> {
        WireValue {
            wire: wv.wire,
            value: match wv.value {
                Some(v) => Some(v),
                None => match wv.wire {
                    Some(k) => match wires.contains_key(k) {
                        true => {
                            Some(wires[k])
                        },
                        false => None
                    },
                    None => unreachable!("no value AND no wire name")
                }
            }
        }
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    gate: Gate,
    lh1: WireValue<'a>,
    lh2: Option<WireValue<'a>>,
    rh: &'a str,
}

impl Instruction<'_> {
    pub fn new(line: &str) -> Instruction {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.len() {
            3 => {
                assert_eq!(tokens[1], "->");
                Instruction {
                    gate: Gate::Assign,
                    lh1: WireValue::new(tokens[0]),
                    lh2: None,
                    rh: tokens[2],
                }
            },
            4 => {
                assert_eq!(tokens[0], "NOT");
                assert_eq!(tokens[2], "->");
                Instruction {
                    gate: Gate::Not,
                    lh1: WireValue::new(tokens[1]),
                    lh2: None,
                    rh: tokens[3],
                }
            },
            5 => {
                assert_eq!(tokens[3], "->");
                Instruction {
                    gate: Gate::new(tokens[1]),
                    lh1: WireValue::new(tokens[0]),
                    lh2: Some(WireValue::new(tokens[2])),
                    rh: tokens[4],
                }
            },
            _ => unreachable!(line)
        }
    }

    pub fn new_assign(value: u16, wire: &str) -> Instruction {
        Instruction {
            gate: Gate::Assign,
            lh1: WireValue::new_value(value),
            lh2: None,
            rh: wire
        }
    }

    pub fn apply<'a>(instr: Instruction<'a>, wires: &Wires) -> Instruction<'a> {
        Instruction {
            gate: instr.gate,
            lh1: WireValue::apply(instr.lh1, wires),
            lh2: match instr.lh2 {
                Some(wv) => Some(WireValue::apply(wv, wires)),
                None => None
            },
            rh: instr.rh
        }
    }

    pub fn all_signals_present(&self) -> bool {
        match self.lh1.value {
            Some(_) => match &self.lh2 {
                Some(wv2) => match wv2.value {
                    Some(_) => true,
                    None => false
                },
                None => true
            },
            None => false
        }
    }
}

fn eval<'a>(instructions: Vec<Instruction<'a>>, wires: &Wires) -> Vec<Instruction<'a>> {
    instructions
        .into_iter()
        .map(|instr| {
            match instr.all_signals_present() {
                true => {
                    match instr.gate {
                        Gate::And => Instruction::new_assign(instr.lh1.value.unwrap() & instr.lh2.as_ref().unwrap().value.unwrap(), instr.rh),
                        Gate::Or => Instruction::new_assign(instr.lh1.value.unwrap() | instr.lh2.as_ref().unwrap().value.unwrap(), instr.rh),
                        Gate::Not => Instruction::new_assign(!instr.lh1.value.unwrap(), instr.rh),
                        Gate::LShift => Instruction::new_assign(instr.lh1.value.unwrap() << instr.lh2.as_ref().unwrap().value.unwrap(), instr.rh),
                        Gate::RShift => Instruction::new_assign(instr.lh1.value.unwrap() >> instr.lh2.as_ref().unwrap().value.unwrap(), instr.rh),
                        _ => instr
                    }
                },
                false => {
                    Instruction::apply(instr, wires)
                }
            }
        })
        .collect()
}

fn run<'a>(instructions: Vec<Instruction<'a>>, wires: &mut Wires<'a>) {
    // break out?
    if instructions.is_empty() {
        return;
    }
    // resolve all fully signalled gates to assignments
    let mut instructions: Vec<Instruction> = eval(instructions, wires);
    // move all assignments to the wires hashtable
    instructions.retain(|instr| {
        match instr.gate {
            Gate::Assign => {
                match instr.lh1.value {
                    Some(v) => {
                        if !wires.contains_key(instr.rh) {
                            wires.insert(instr.rh, v);
                        }
                        false // drop
                    },
                    None => true // retain
                }
            }
            _ => true // retain
        }
    });
    run(instructions, wires);
}

fn parse(lines: &Vec<String>) -> Vec<Instruction> {
    lines.iter()
        .map(|s| Instruction::new(s))
        .collect()
}

fn main() {
    let data = load("data/day7.txt");
    let instructions = parse(&data);
    let mut wires: Wires = HashMap::new();
    run(instructions, &mut wires);

    let a_value = *wires.get("a").expect("should have value");
    assert_eq!(a_value, 956);

    let instructions = parse(&data);
    let mut wires: Wires = HashMap::new();
    wires.insert("b", a_value);
    run(instructions, &mut wires);
    assert_eq!(*wires.get("a").expect("should have value"), 40149);
}
