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
    let mut people: Vec<_> = input
        .lines()
        .map(|l| l.split_once(" would ").unwrap().0)
        .collect::<HashSet<_>>()
        .drain()
        .collect();

    let happiness: HashMap<_, _> = input
        .lines()
        .map(|l| l.split_once(" would ").unwrap())
        .map(|(l, r)| {
            (
                l,
                r.split_once(" happiness units by sitting next to ")
                    .unwrap(),
            )
        })
        .map(|(l, (h, r))| ((l, r.trim_end_matches('.')), h))
        .map(|((l, r), h)| match h {
            _ if h.starts_with("gain") => ((l, r), h[5..].parse::<i32>().unwrap()),
            _ if h.starts_with("lose") => ((l, r), -h[5..].parse::<i32>().unwrap()),
            _ => unreachable!(),
        })
        .collect();

    // Part 1
    let mut max_happiness = 0;
    let mut iter = people.permute();
    while let Some(perm) = iter.next() {
        let happiness = perm
            .windows(2)
            .map(|w| happiness.get(&(w[0], w[1])).unwrap() + happiness.get(&(w[1], w[0])).unwrap())
            .sum::<i32>()
            + happiness.get(&(perm[0], perm[perm.len() - 1])).unwrap()
            + happiness.get(&(perm[perm.len() - 1], perm[0])).unwrap();
        max_happiness = max_happiness.max(happiness);
    }
    println!("Part 1: {}", max_happiness);

    // Part 2
    people.push("Me");
    let mut max_happiness = 0;
    let mut iter = people.permute();
    while let Some(perm) = iter.next() {
        let happiness = perm
            .windows(2)
            .map(|w| {
                happiness.get(&(w[0], w[1])).unwrap_or(&0)
                    + happiness.get(&(w[1], w[0])).unwrap_or(&0)
            })
            .sum::<i32>()
            + happiness
                .get(&(perm[0], perm[perm.len() - 1]))
                .unwrap_or(&0)
            + happiness
                .get(&(perm[perm.len() - 1], perm[0]))
                .unwrap_or(&0);
        max_happiness = max_happiness.max(happiness);
    }
    println!("Part 2: {}", max_happiness);
}
