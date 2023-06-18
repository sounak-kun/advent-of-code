use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

#[derive(Debug, PartialEq)]
enum Json<'a> {
    Number(i32),
    String(&'a str),
    Array(Vec<Json<'a>>),
    Object(HashMap<&'a str, Json<'a>>),
}

impl<'a, 'b: 'a> Json<'a> {
    fn from_str(data: &'b str) -> Option<Json<'a>> {
        Self::parse(&mut Tokenizer::new(data))
    }

    fn parse(data: &mut Tokenizer<'b>) -> Option<Json<'a>> {
        match data.next()? {
            "{" => {
                let mut object = HashMap::new();
                if data.peek()? == "}" {
                    data.next()?;
                    return Some(Json::Object(object));
                }
                loop {
                    let key = data.next()?.trim_matches('"');
                    if data.next()? != ":" {
                        return None;
                    }
                    let value = Self::parse(data)?;
                    object.insert(key, value);
                    match data.next()? {
                        "," => continue,
                        "}" => break Some(Json::Object(object)),
                        _ => return None,
                    }
                }
            }
            "[" => {
                let mut array = Vec::new();
                if data.peek()? == "]" {
                    data.next()?;
                    return Some(Json::Array(array));
                }
                loop {
                    let value = Self::parse(data)?;
                    array.push(value);
                    match data.next()? {
                        "," => continue,
                        "]" => break Some(Json::Array(array)),
                        _ => return None,
                    }
                }
            }
            token => token.parse().map_or_else(
                |_| Some(Json::String(token.trim_matches('"'))),
                |n| Some(Json::Number(n)),
            ),
        }
    }
}

struct Tokenizer<'a> {
    data: &'a str,
}

impl<'a: 'b, 'b> Tokenizer<'a> {
    fn new(data: &'a str) -> Tokenizer<'b> {
        Tokenizer { data }
    }

    fn next(&'b mut self) -> Option<&'a str> {
        self.data
            .trim_start()
            .find(['{', '}', '[', ']', ',', ':'])
            .map(|i| match i {
                0 => {
                    let (token, rest) = self.data.split_at(1);
                    self.data = rest;
                    token
                }
                i => {
                    let (token, rest) = self.data.split_at(i);
                    self.data = rest;
                    token.trim_end()
                }
            })
    }

    fn peek(&'b self) -> Option<&'a str> {
        self.data
            .trim_start()
            .find(['{', '}', '[', ']', ',', ':'])
            .map(|i| match i {
                0 => self.data.split_at(1).0,
                i => self.data.split_at(i).0.trim_end(),
            })
    }
}

impl Json<'_> {
    fn sum(&self) -> i32 {
        match self {
            Json::Number(n) => *n,
            Json::String(_) => 0,
            Json::Array(a) => a.iter().map(Self::sum).sum(),
            Json::Object(o) => o.values().map(Self::sum).sum(),
        }
    }

    fn sum_without_red(&self) -> i32 {
        match self {
            Json::Number(n) => *n,
            Json::String(_) => 0,
            Json::Array(a) => a.iter().map(Self::sum_without_red).sum(),
            Json::Object(o) => {
                if o.values().any(|v| matches!(v, Json::String("red"))) {
                    0
                } else {
                    o.values().map(Self::sum_without_red).sum()
                }
            }
        }
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let json = Json::from_str(&input).unwrap();

    // Part 1
    println!("Part 1: {}", json.sum());

    // Part 2
    println!("Part 2: {}", json.sum_without_red());
}
