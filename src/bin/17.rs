use std::io::{self, prelude::*};

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let containers: Vec<i32> = input.trim().lines().map(|l| l.parse().unwrap()).collect();

    // Part 1
    const TARGET: i32 = 150;
    let mut last = 0;
    let mut sum = 0;
    let combinations = (1..1u32 << containers.len())
        .filter(|&mask| {
            let mask = mask ^ (mask >> 1);
            let added = mask & !last;
            let removed = !mask & last;
            last = mask;
            match (added, removed) {
                (0, _) => sum -= containers[removed.trailing_zeros() as usize],
                (_, 0) => sum += containers[added.trailing_zeros() as usize],
                _ => unreachable!(),
            }
            sum == TARGET
        })
        .count();
    println!("Part 1: {}", combinations);

    // Part 2
    let mut last = 0;
    let mut sum = 0;
    let combinations = (1..1u32 << containers.len())
        .filter_map(|mut mask| {
            mask = mask ^ (mask >> 1);
            let added = mask & !last;
            let removed = !mask & last;
            last = mask;
            match (added, removed) {
                (_, 0) => sum += containers[added.trailing_zeros() as usize],
                (0, _) => sum -= containers[removed.trailing_zeros() as usize],
                _ => unreachable!(),
            }
            match sum {
                TARGET => Some(mask.count_ones()),
                _ => None,
            }
        })
        .fold((u32::MAX, 0), |(min, count), n| match n {
            _ if n == min => (min, count + 1),
            _ if n < min => (n, 1),
            _ => (min, count),
        });
    println!("Part 2: {}", combinations.1);
}
