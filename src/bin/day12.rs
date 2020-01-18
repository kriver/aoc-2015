use aoc_2015::load;
use std::fmt::{Debug, Write};
use array_macro::__core::fmt::{Formatter, Error};
use std::iter::Peekable;
use std::str::Chars;
use array_macro::__core::iter::Enumerate;

#[derive(Debug)]
enum TokenType {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Number,
    String
}

union Value<'a> {
    num: i32,
    str: &'a str,
}

impl<'a> Value<'a> {
    fn num(&self) -> i32 {
        unsafe { self.num }
    }

    fn str(&self) -> &'a str {
        unsafe { self.str }
    }
}

struct Token<'a> {
    tp: TokenType,
    value: Option<Value<'a>>,
}

impl<'a> Token<'a> {
    fn simple(tp: TokenType) -> Self {
        Token { tp, value: None }
    }

    fn string(s: &'a str) -> Self {
        Token {
            tp: TokenType::String,
            value: Some(Value { str: s }),
        }
    }

    fn number(n: i32) -> Self {
        Token {
            tp: TokenType::Number,
            value: Some(Value { num: n }),
        }
    }

    fn is_number(&self) -> bool {
        match self.tp {
            TokenType::Number => true,
            _ => false,
        }
    }
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.tp {
            TokenType::ObjectStart => f.write_char('{'),
            TokenType::ObjectEnd => f.write_char('}'),
            TokenType::ArrayStart => f.write_char('['),
            TokenType::ArrayEnd => f.write_char(']'),
            TokenType::Number => f.write_fmt(format_args!("Num: {}", self.value.as_ref().unwrap().num())),
            TokenType::String => f.write_fmt(format_args!("Str: '{}'", self.value.as_ref().unwrap().str())),
        }
    }
}

struct JsonIter<'a> {
    str: &'a str,
    chars: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> JsonIter<'a> {
    fn new(s: &'a str) -> Self {
        JsonIter {
            str: s,
            chars: s.chars().enumerate().peekable(),
        }
    }

    fn reset(&mut self) {
        self.chars = self.str.chars().enumerate().peekable();
    }

    fn eat_string(&mut self, start: usize) -> &'a str {
        let mut end = start + 1;
        loop {
            match self.chars.peek() {
                Some((_, ch)) => match ch {
                    'a'..='z' => {
                        self.chars.next();
                        end += 1;
                    },
                    _ => return &self.str[start..end]
                }
                None => return &self.str[start..]
            }
        }
    }

    fn eat_number(&mut self, start: usize) -> i32 {
        let mut end = start + 1;
        loop {
            match self.chars.peek() {
                Some((_, ch)) => match ch {
                    '0'..='9' => {
                        self.chars.next();
                        end += 1;
                    },
                    _ => return self.str[start..end].parse::<i32>().unwrap()
                }
                None =>
                    return self.str[start..].parse::<i32>().unwrap()
            }
        }
    }
}

impl<'a> Iterator for JsonIter<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some((i, ch)) => match ch {
                '{' => Some(Token::simple(TokenType::ObjectStart)),
                '}' => Some(Token::simple(TokenType::ObjectEnd)),
                '[' => Some(Token::simple(TokenType::ArrayStart)),
                ']' => Some(Token::simple(TokenType::ArrayEnd)),
                'a'..='z' => Some(Token::string(self.eat_string(i))),
                '-' | '0'..='9' => Some(Token::number(self.eat_number(i))),
                '"' | ':' | ',' => self.next(), // skip
                _ => unreachable!()
            },
            None => None
        }
    }
}

fn sum_numbers(json: &mut JsonIter) -> i32 {
    let mut sum = 0;
    for token in json {
        if token.is_number() {
            sum += token.value.unwrap().num()
        }
    }
    sum
}

fn sum_except_red(json: &mut JsonIter) -> i32 {
    fn recurse(json: &mut JsonIter) -> i32 {
        let mut has_red = false;
        let mut in_array = 0;
        let mut sum = 0;
        while let Some(token) = json.next() {
            match token.tp {
                TokenType::Number => sum += token.value.unwrap().num(),
                TokenType::ObjectStart => sum += recurse(json),
                TokenType::ObjectEnd => break,
                TokenType::ArrayStart=> in_array += 1,
                TokenType::ArrayEnd=> in_array -= 1,
                TokenType::String => has_red |= in_array == 0 && token.value.unwrap().str() == "red",
            }
        }
        if has_red { 0 } else { sum }
    }
    recurse(json)
}

fn main() {
    assert_eq!(sum_except_red(&mut JsonIter::new(r#"[1,2,3]"#)), 6);
    assert_eq!(sum_except_red(&mut JsonIter::new(r#"[1,{"c":"red","b":2},3]"#)), 4);
    assert_eq!(sum_except_red(&mut JsonIter::new(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)), 0);
    assert_eq!(sum_except_red(&mut JsonIter::new(r#"[1,"red",5]"#)), 6);

    let data = load("data/day12.txt");
    let mut json = JsonIter::new(&data[0]);
    assert_eq!(sum_numbers(&mut json), 119_433);
    json.reset();
    assert_eq!(sum_except_red(&mut json), 68_466);
}
