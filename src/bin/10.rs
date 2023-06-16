use std::io::{self, prelude::*};

fn look_and_say(string: &str) -> String {
    // Add terminator to process last character
    string
        .chars()
        // Add terminator to process last character
        .chain(std::iter::once('\0'))
        .fold(
            (String::new(), None, 0),
            |(mut string, prev, count), c| match prev {
                Some(prev_character) if prev_character == c => (string, prev, count + 1),
                Some(prev_character) => {
                    string.push_str(&count.to_string());
                    string.push(prev_character);
                    (string, Some(c), 1)
                }
                None => (string, Some(c), 1),
            },
        )
        .0
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let input = input.trim();

    // Part 1
    let mut string = input.to_string();
    for _ in 0..40 {
        string = look_and_say(&string);
    }
    println!("Part 1: {}", string.len());

    // Part 2
    let mut string = input.to_string();
    for _ in 0..50 {
        string = look_and_say(&string);
    }
    println!("Part 2: {}", string.len());
}
