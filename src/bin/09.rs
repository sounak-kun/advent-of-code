use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*};

trait LendingIterator {
    type Item<'i>
    where
        Self: 'i;

    fn next<'i>(&'i mut self) -> Option<Self::Item<'i>>;
}

struct Permutations<'a, T> {
    data: &'a mut [T],
    states: Option<Vec<u8>>,
    directions: Option<Vec<i8>>,
}

impl<'a, T> LendingIterator for Permutations<'a, T> {
    type Item<'i> = &'i [T]
    where
        Self: 'i;

    fn next<'i>(&'i mut self) -> Option<Self::Item<'i>> {
        // Initialize states and directions
        let (Some(states), Some(directions)) = (self.states.as_mut(), self.directions.as_mut())
        else {
            self.states = Some((0..self.data.len()).map(|s| s as u8).collect());
            self.directions = Some((0..self.data.len()).map(|_| -1).collect());
            return Some(self.data);
        };

        // Find the largest mobile element
        let mobile_element = states
            .windows(2)
            .zip(directions.windows(2))
            .enumerate()
            .filter_map(|(i, (swin, dwin))| match dwin {
                [1, _] if swin[0] > swin[1] => Some((i, i + 1, swin[0])),
                [_, -1] if swin[0] < swin[1] => Some((i + 1, i, swin[1])),
                _ => None,
            })
            .max_by_key(|(_, _, s)| *s)?;

        // Swap the largest mobile element
        self.data.swap(mobile_element.0, mobile_element.1);
        states.swap(mobile_element.0, mobile_element.1);
        directions.swap(mobile_element.0, mobile_element.1);

        // Reverse the direction of all elements larger than the largest mobile element
        states.iter().zip(directions.iter_mut()).for_each(|(s, d)| {
            if *s > mobile_element.2 {
                *d = -*d;
            }
        });

        Some(self.data)
    }
}

trait Permute<'a, T> {
    fn permute(self) -> Permutations<'a, T>;
}

impl<'a, T> Permute<'a, T> for &'a mut [T] {
    fn permute(self) -> Permutations<'a, T> {
        Permutations {
            data: self,
            states: None,
            directions: None,
        }
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let mut locations = HashSet::new();
    let mut distances = HashMap::new();
    input.lines().for_each(|line| {
        let mut words = line.split_whitespace();
        let from = words.nth(0).unwrap();
        let to = words.nth(1).unwrap();
        let distance = words.nth(1).unwrap().parse::<usize>().unwrap();
        locations.insert(from);
        locations.insert(to);
        if from < to {
            distances.insert((from, to), distance);
        } else {
            distances.insert((to, from), distance);
        }
    });
    let mut locations = Vec::from_iter(locations.drain());

    // Part 1
    let mut shortest_distance = usize::MAX;
    let mut iter = locations.permute();
    while let Some(route) = iter.next() {
        shortest_distance = shortest_distance.min(
            route
                .windows(2)
                .map(|pair| {
                    if pair[0] < pair[1] {
                        distances.get(&(pair[0], pair[1])).unwrap()
                    } else {
                        distances.get(&(pair[1], pair[0])).unwrap()
                    }
                })
                .sum::<usize>(),
        );
    }
    println!("Part 1: {}", shortest_distance);

    // Part 2
    let mut longest_distance = usize::MIN;
    let mut iter = locations.permute();
    while let Some(route) = iter.next() {
        longest_distance = longest_distance.max(
            route
                .windows(2)
                .map(|pair| {
                    if pair[0] < pair[1] {
                        distances.get(&(pair[0], pair[1])).unwrap()
                    } else {
                        distances.get(&(pair[1], pair[0])).unwrap()
                    }
                })
                .sum::<usize>(),
        );
    }
    println!("Part 2: {}", longest_distance);
}
