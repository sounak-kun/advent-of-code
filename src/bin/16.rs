use std::io::{self, prelude::*};

#[derive(Default)]
struct AuntSue {
    children: Property<u8>,
    cats: Property<u8>,
    samoyeds: Property<u8>,
    pomeranians: Property<u8>,
    akitas: Property<u8>,
    vizslas: Property<u8>,
    goldfish: Property<u8>,
    trees: Property<u8>,
    cars: Property<u8>,
    perfumes: Property<u8>,
}

enum Property<T> {
    Known(T),
    Unknown,
}
use Property::*;

impl<T> Default for Property<T> {
    fn default() -> Self {
        Self::Unknown
    }
}

impl<T: PartialEq> PartialEq for Property<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Known(a), Self::Known(b)) => a == b,
            _ => true,
        }
    }
}

impl<T: PartialOrd> PartialOrd for Property<T> {
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Known(a), Self::Known(b)) => a < b,
            _ => true,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Known(a), Self::Known(b)) => a > b,
            _ => true,
        }
    }

    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        unreachable!("This method is never called")
    }
}

impl AuntSue {
    fn eq_one(&self, target: &Self) -> bool {
        // These could be rewritten using Option<T> like this:
        // self.propetry.xor(target.propetry).is_some() || self.propetry == target.propetry
        self.children == target.children
            && self.cats == target.cats
            && self.samoyeds == target.samoyeds
            && self.pomeranians == target.pomeranians
            && self.akitas == target.akitas
            && self.vizslas == target.vizslas
            && self.goldfish == target.goldfish
            && self.trees == target.trees
            && self.cars == target.cars
            && self.perfumes == target.perfumes
    }
    fn eq_two(&self, target: &Self) -> bool {
        self.children == target.children
            && self.cats > target.cats
            && self.samoyeds == target.samoyeds
            && self.pomeranians < target.pomeranians
            && self.akitas == target.akitas
            && self.vizslas == target.vizslas
            && self.goldfish < target.goldfish
            && self.trees > target.trees
            && self.cars == target.cars
            && self.perfumes == target.perfumes
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    const TARGET: AuntSue = AuntSue {
        children: Known(3),
        cats: Known(7),
        samoyeds: Known(2),
        pomeranians: Known(3),
        akitas: Known(0),
        vizslas: Known(0),
        goldfish: Known(5),
        trees: Known(3),
        cars: Known(2),
        perfumes: Known(1),
    };

    // Parse input
    let sues: Vec<_> = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1)
        .map(|l| {
            let properties = l.split(',');
            let mut sue = AuntSue::default();
            properties.for_each(|p| {
                let (property, value) = p.split_once(':').unwrap();
                let property = property.trim();
                let value = value.trim().parse().unwrap();
                match property {
                    "children" => sue.children = Known(value),
                    "cats" => sue.cats = Known(value),
                    "samoyeds" => sue.samoyeds = Known(value),
                    "pomeranians" => sue.pomeranians = Known(value),
                    "akitas" => sue.akitas = Known(value),
                    "vizslas" => sue.vizslas = Known(value),
                    "goldfish" => sue.goldfish = Known(value),
                    "trees" => sue.trees = Known(value),
                    "cars" => sue.cars = Known(value),
                    "perfumes" => sue.perfumes = Known(value),
                    _ => unreachable!(),
                }
            });
            sue
        })
        .collect();

    // Part 1
    let answer = sues
        .iter()
        .enumerate()
        .find(|(_, s)| s.eq_one(&TARGET))
        .map(|(i, _)| i + 1)
        .unwrap();
    println!("Part 1: {}", answer);

    // Part 2
    let answer = sues
        .iter()
        .enumerate()
        .find(|(_, s)| s.eq_two(&TARGET))
        .map(|(i, _)| i + 1)
        .unwrap();
    println!("Part 2: {}", answer);
}
