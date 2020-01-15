fn look_and_say_vec(input: &[u32]) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();
    let mut num: u32 = 0;
    let mut prev: Option<u32> = None;
    for n in input {
        match prev {
            Some(p) =>
                if p == *n {
                    num += 1;
                } else {
                    output.push(num);
                    output.push(p);
                    prev = Some(*n);
                    num = 1;
                },
            None => {
                prev = Some(*n);
                num = 1;
            }
        }
    }
    output.push(num);
    output.push(prev.unwrap());
    output
}

fn to_vec(digits: &str) -> Vec<u32> {
    digits.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn from_vec(digits: &[u32]) -> String {
    digits.iter()
        .map(|i| i.to_string())
        .fold("".to_string(), |acc, s| acc + s.as_str())
}

fn look_and_say(digits: &str) -> String {
    let input = to_vec(digits);
    let output = look_and_say_vec(&input);
    from_vec(&output)
}

fn look_and_say_loop(digits: &str, count: usize) -> String {
    let mut input = to_vec(digits);
    for _ in 0..count {
        input = look_and_say_vec(&input);
    }
    from_vec(&input)
}

fn main() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");

    assert_eq!(look_and_say_loop("1", 5), "312211");
    assert_eq!(look_and_say_loop("1113222113", 40).len(), 252_594);
    assert_eq!(look_and_say_loop("1113222113", 50).len(), 3_579_328);
}
