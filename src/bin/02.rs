use std::io::{self, prelude::*};

type Cuboid = [i32; 3];

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let mut presents: Vec<Cuboid> = input
        .lines()
        .map(|line| {
            line.splitn(3, 'x')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .expect("Input should only contain 3 integers per line")
        })
        .collect();

    presents
        .iter_mut()
        .for_each(|cuboid| cuboid.sort_unstable());

    // Part 1
    let paper = presents
        .iter()
        .map(|[a, b, c]| 2 * (a * b + b * c + c * a) + a * b)
        .sum::<i32>();
    println!("Part 1: {}", paper);

    // Part 2
    let ribbon = presents
        .iter()
        .map(|[a, b, c]| 2 * (a + b) + a * b * c)
        .sum::<i32>();
    println!("Part 2: {}", ribbon);
}
