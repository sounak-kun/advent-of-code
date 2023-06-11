use std::io::{self, prelude::*};
use std::ops::ControlFlow::{Break, Continue};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let nice = input
        .lines()
        .filter(|line| {
            line.bytes()
                .try_fold(0, |acc, c| match c {
                    b'a' | b'e' | b'i' | b'o' | b'u' => {
                        if acc == 2 {
                            Break(3)
                        } else {
                            Continue(acc + 1)
                        }
                    }
                    _ => Continue(acc),
                })
                .is_break()
        })
        .filter(|line| line.as_bytes().windows(2).any(|w| w[0] == w[1]))
        .filter(|line| {
            !line.contains("ab")
                && !line.contains("cd")
                && !line.contains("pq")
                && !line.contains("xy")
        })
        .count();
    // This is much easier to do with regex
    // cat input | rg '([aeiou].*){3}' | rg -P '(.)\1' | rg -v 'ab|cd|pq|xy' | wc -l
    println!("Part 1: {}", nice);

    // Part 2
    let nice = input
        .lines()
        .filter(|line| {
            line.as_bytes()
                .windows(2)
                .enumerate()
                // (i + 2) to avoid overlapping
                .any(|(i, w)| line.as_bytes().windows(2).skip(i + 2).any(|v| w == v))
        })
        .filter(|line| line.as_bytes().windows(3).any(|w| w[0] == w[2]))
        .count();
    // This is much easier to do with regex
    // cat input | rg -P '(..).*\1' | rg -P '(.).\1' | wc -l
    println!("Part 2: {}", nice);
}
