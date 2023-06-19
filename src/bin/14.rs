use std::io::{self, prelude::*};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let horses: Vec<(i32, i32, i32)> = input
        .lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            (
                words.nth(3).unwrap().parse().unwrap(),
                words.nth(2).unwrap().parse().unwrap(),
                words.nth(6).unwrap().parse().unwrap(),
            )
        })
        .collect();

    // Part 1
    let max_distance = horses
        .iter()
        .map(|(s, t, r)| {
            let cycles = 2503 / (t + r);
            let remainder = 2503 % (t + r);
            (cycles * t + remainder.min(*t)) * s
        })
        .max()
        .unwrap();
    println!("Part 1: {}", max_distance);

    // Part 2
    let mut scores = vec![0; horses.len()];
    for i in 1..=2503 {
        let mut max_distance = 0;
        let mut max_index = 0;
        for (index, (s, t, r)) in horses.iter().enumerate() {
            let cycles = i / (t + r);
            let remainder = i % (t + r);
            let distance = (cycles * t + remainder.min(*t)) * s;
            if distance > max_distance {
                max_distance = distance;
                max_index = index;
            }
        }
        scores[max_index] += 1;
    }
    println!("Part 2: {}", scores.iter().max().unwrap());
}
