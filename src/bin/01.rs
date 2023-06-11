use std::io::{self, prelude::*};
use std::ops::ControlFlow::{Break, Continue};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let floor = input.bytes().fold(0, |acc, c| match c {
        b'(' => acc + 1,
        b')' => acc - 1,
        _ => unreachable!("Input should only contain ( and )"),
    });
    println!("Part 1: {}", floor);

    // Part 2
    let Break(basement) = input
        .bytes()
        .enumerate()
        .try_fold(0, |acc, (i, c)| match c {
            b'(' => Continue(acc + 1),
            b')' if acc == 0 => Break(i + 1),
            b')' => Continue(acc - 1),
            _ => unreachable!("Input should only contain ( and )"),
        }) else {
            unreachable!("Should always hit basement according to problem statement")
        };
    println!("Part 2: {}", basement);
}
