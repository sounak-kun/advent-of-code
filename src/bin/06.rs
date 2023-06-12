use std::io::{self, prelude::*};

enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    operation: Operation,
    start: (usize, usize),
    end: (usize, usize),
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let instructions: Vec<_> = input
        .lines()
        .map(|line| match line {
            _ if line.starts_with("turn on") => (Operation::TurnOn, &line[8..]),
            _ if line.starts_with("turn off") => (Operation::TurnOff, &line[9..]),
            _ if line.starts_with("toggle") => (Operation::Toggle, &line[7..]),
            _ => unreachable!("Input should only start with 'turn on', 'turn off' or 'toggle'"),
        })
        .map(|(operation, line)| {
            let parts: Vec<Vec<_>> = line
                .splitn(2, " through ")
                .map(|part| {
                    part.splitn(2, ',')
                        .map(|coord| coord.parse().unwrap())
                        .collect()
                })
                .collect();
            debug_assert!(parts[0][0] <= parts[1][0]);
            debug_assert!(parts[0][1] <= parts[1][1]);
            Instruction {
                operation,
                start: (parts[0][0], parts[0][1]),
                end: (parts[1][0], parts[1][1]),
            }
        })
        .collect();

    // Part 1
    let mut lights = vec![vec![false; 1000]; 1000];
    instructions
        .iter()
        .map(|instruction| -> (_, fn(&mut bool)) {
            match instruction.operation {
                Operation::TurnOn => (instruction, |l| *l = true),
                Operation::TurnOff => (instruction, |l| *l = false),
                Operation::Toggle => (instruction, |l| *l = !*l),
            }
        })
        .for_each(|(instruction, operation)| {
            lights
                .get_mut(instruction.start.0..=instruction.end.0)
                .unwrap()
                .iter_mut()
                .for_each(|row| {
                    row.get_mut(instruction.start.1..=instruction.end.1)
                        .unwrap()
                        .iter_mut()
                        .for_each(&operation)
                })
        });
    println!(
        "Part 1: {}",
        lights
            .iter()
            .flatten()
            .fold(0, |acc, l| if *l { acc + 1 } else { acc })
    );

    // Part 2
    let mut lights = vec![vec![0; 1000]; 1000];
    instructions
        .iter()
        .map(|instruction| -> (_, fn(&mut u32)) {
            match instruction.operation {
                Operation::TurnOn => (instruction, |l| *l += 1),
                // Saturating subtraction to avoid underflow
                Operation::TurnOff => (instruction, |l| *l = l.saturating_sub(1)),
                Operation::Toggle => (instruction, |l| *l += 2),
            }
        })
        .for_each(|(instruction, operation)| {
            lights
                .get_mut(instruction.start.0..=instruction.end.0)
                .unwrap()
                .iter_mut()
                .for_each(|row| {
                    row.get_mut(instruction.start.1..=instruction.end.1)
                        .unwrap()
                        .iter_mut()
                        .for_each(&operation)
                })
        });
    println!("Part 1: {}", lights.iter().flatten().sum::<u32>());
}
