use std::io::{self, prelude::*};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let code_length = input.lines().map(|line| line.len()).sum::<usize>();
    let memory_length = input
        .lines()
        .map(|line| {
            // Subtract 2 for the surrounding quotes
            let mut length = line.len() - 2;
            let mut skip_next = false;
            line[1..line.len() - 1]
                .as_bytes()
                .windows(2)
                .for_each(|w| match w {
                    _ if skip_next => skip_next = false,
                    b"\\\"" | b"\\\\" => {
                        length -= 1;
                        skip_next = true;
                    }
                    b"\\x" => length -= 3,
                    _ => (),
                });
            length
        })
        .sum::<usize>();
    println!("Part 1: {}", code_length - memory_length);

    // Part 2
    let memory_length = input.lines().map(|line| line.len()).sum::<usize>();
    let code_length = input
        .lines()
        .map(|line| {
            // Add 2 for the surrounding quotes
            let mut length = line.len() + 2;
            line.as_bytes().iter().for_each(|b| match b {
                b'"' | b'\\' => length += 1,
                _ => (),
            });
            length
        })
        .sum::<usize>();
    println!("Part 2: {}", code_length - memory_length);
}
