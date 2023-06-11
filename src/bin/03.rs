use std::collections::HashSet;
use std::io::{self, prelude::*};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let mut visited = HashSet::new();
    let mut position = (0, 0);
    visited.insert(position);

    input.bytes().for_each(|c| {
        match c {
            b'^' => position.1 += 1,
            b'v' => position.1 -= 1,
            b'>' => position.0 += 1,
            b'<' => position.0 -= 1,
            _ => unreachable!("Input should only contain ^, v, > or <"),
        }
        visited.insert(position);
    });
    println!("Part 1: {}", visited.len());

    // Part 2
    let mut visited = HashSet::new();
    let mut santa = (0, 0);
    let mut robot = (0, 0);
    visited.insert(santa);

    input.as_bytes().chunks_exact(2).for_each(|chunk| {
        match chunk[0] {
            b'^' => santa.1 += 1,
            b'v' => santa.1 -= 1,
            b'>' => santa.0 += 1,
            b'<' => santa.0 -= 1,
            _ => unreachable!("Input should only contain ^, v, > or <"),
        }
        match chunk[1] {
            b'^' => robot.1 += 1,
            b'v' => robot.1 -= 1,
            b'>' => robot.0 += 1,
            b'<' => robot.0 -= 1,
            _ => unreachable!("Input should only contain ^, v, > or <"),
        }
        visited.insert(santa);
        visited.insert(robot);
    });
    println!("Part 2: {}", visited.len());
}
