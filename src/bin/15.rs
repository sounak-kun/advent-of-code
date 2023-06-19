use std::io::{self, prelude::*};

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let ingredients: Vec<_> = input
        .lines()
        .map(|l| {
            let mut words = l.split([' ', ',', ':']);
            Ingredient {
                capacity: words.nth(3).unwrap().parse().unwrap(),
                durability: words.nth(2).unwrap().parse().unwrap(),
                flavor: words.nth(2).unwrap().parse().unwrap(),
                texture: words.nth(2).unwrap().parse().unwrap(),
                calories: words.nth(2).unwrap().parse().unwrap(),
            }
        })
        .collect();

    // Part 1
    // Brute force all possible combinations of ingredients
    // The number of combinations is 103C3 = 176851
    // This can be calculated using the stars and bars method
    let max_score = (0..100)
        .map(|a| {
            (0..100 - a)
                .map(|b| {
                    (0..100 - a - b)
                        .map(|c| {
                            let d = 100 - a - b - c;
                            let capacity = a * ingredients[0].capacity
                                + b * ingredients[1].capacity
                                + c * ingredients[2].capacity
                                + d * ingredients[3].capacity;
                            let durability = a * ingredients[0].durability
                                + b * ingredients[1].durability
                                + c * ingredients[2].durability
                                + d * ingredients[3].durability;
                            let flavor = a * ingredients[0].flavor
                                + b * ingredients[1].flavor
                                + c * ingredients[2].flavor
                                + d * ingredients[3].flavor;
                            let texture = a * ingredients[0].texture
                                + b * ingredients[1].texture
                                + c * ingredients[2].texture
                                + d * ingredients[3].texture;
                            capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0)
                        })
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("Part 1: {}", max_score);

    // Part 2
    let max_score = (0..100)
        .map(|a| {
            (0..100 - a)
                .map(|b| {
                    (0..100 - a - b)
                        .map(|c| {
                            let d = 100 - a - b - c;
                            let calories = a * ingredients[0].calories
                                + b * ingredients[1].calories
                                + c * ingredients[2].calories
                                + d * ingredients[3].calories;
                            if calories != 500 {
                                return 0;
                            }
                            let capacity = a * ingredients[0].capacity
                                + b * ingredients[1].capacity
                                + c * ingredients[2].capacity
                                + d * ingredients[3].capacity;
                            let durability = a * ingredients[0].durability
                                + b * ingredients[1].durability
                                + c * ingredients[2].durability
                                + d * ingredients[3].durability;
                            let flavor = a * ingredients[0].flavor
                                + b * ingredients[1].flavor
                                + c * ingredients[2].flavor
                                + d * ingredients[3].flavor;
                            let texture = a * ingredients[0].texture
                                + b * ingredients[1].texture
                                + c * ingredients[2].texture
                                + d * ingredients[3].texture;
                            capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0)
                        })
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("Part 2: {}", max_score);
}
