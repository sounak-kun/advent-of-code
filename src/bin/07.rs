use std::collections::HashMap;
use std::io::{self, prelude::*};

struct Instruction<'a> {
    operation: Operator<'a>,
    output: &'a str,
}

enum Operator<'a> {
    NOP(Operand<'a>),
    AND(Operand<'a>, Operand<'a>),
    OR(Operand<'a>, Operand<'a>),
    LSHIFT(Operand<'a>, Operand<'a>),
    RSHIFT(Operand<'a>, Operand<'a>),
    NOT(Operand<'a>),
}

enum Operand<'a> {
    Wire(&'a str),
    Value(u16),
}

impl<'a> Operand<'a> {
    fn new(input: &'a str) -> Self {
        input.parse().map_or(Operand::Wire(input), Operand::Value)
    }
}

type WireState<'a> = Result<u16, &'a Instruction<'a>>;

struct Circuit<'a> {
    circuit: HashMap<&'a str, WireState<'a>>,
}

impl<'a> Circuit<'a> {
    fn new() -> Self {
        Self {
            circuit: HashMap::new(),
        }
    }

    fn run(&mut self, instruction: &'a Instruction<'a>) {
        match &instruction.operation {
            Operator::NOP(a) | Operator::NOT(a) => {
                let a = match a {
                    Operand::Wire(wire) => self.get(wire),
                    Operand::Value(value) => Some(*value),
                };
                let Some(a) = a else {
                    self.circuit.insert(instruction.output, Err(instruction));
                    return;
                };
                self.circuit.insert(
                    instruction.output,
                    match instruction.operation {
                        Operator::NOP(_) => Ok(a),
                        Operator::NOT(_) => Ok(!a),
                        _ => unreachable!(),
                    },
                );
            }
            Operator::AND(a, b)
            | Operator::OR(a, b)
            | Operator::LSHIFT(a, b)
            | Operator::RSHIFT(a, b) => {
                let a = match a {
                    Operand::Wire(wire) => self.get(wire),
                    Operand::Value(value) => Some(*value),
                };
                let Some(a) = a else {
                    self.circuit.insert(instruction.output, Err(instruction));
                    return;
                };
                let b = match b {
                    Operand::Wire(wire) => self.get(wire),
                    Operand::Value(value) => Some(*value),
                };
                let Some(b) = b else {
                    self.circuit.insert(instruction.output, Err(instruction));
                    return;
                };
                self.circuit.insert(
                    instruction.output,
                    match instruction.operation {
                        Operator::AND(_, _) => Ok(a & b),
                        Operator::OR(_, _) => Ok(a | b),
                        Operator::LSHIFT(_, _) => Ok(a << b),
                        Operator::RSHIFT(_, _) => Ok(a >> b),
                        _ => unreachable!(),
                    },
                );
            }
        }
    }

    fn get(&mut self, wire: &'a str) -> Option<u16> {
        match self.circuit.get(wire)? {
            Ok(value) => Some(*value),
            Err(instruction) => {
                self.run(instruction);
                self.circuit.get(wire)?.ok()
            }
        }
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    use Operator::*;
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(" -> ").collect();
            let output = parts[1];
            let operation = match parts[0] {
                part if part.contains("AND") => {
                    let parts: Vec<_> = part.splitn(2, " AND ").collect();
                    AND(Operand::new(parts[0]), Operand::new(parts[1]))
                }
                part if part.contains("OR") => {
                    let parts: Vec<_> = part.splitn(2, " OR ").collect();
                    OR(Operand::new(parts[0]), Operand::new(parts[1]))
                }
                part if part.contains("LSHIFT") => {
                    let parts: Vec<_> = part.splitn(2, " LSHIFT ").collect();
                    LSHIFT(Operand::new(parts[0]), Operand::new(parts[1]))
                }
                part if part.contains("RSHIFT") => {
                    let parts: Vec<_> = part.splitn(2, " RSHIFT ").collect();
                    RSHIFT(Operand::new(parts[0]), Operand::new(parts[1]))
                }
                part if part.contains("NOT") => NOT(Operand::new(&part[4..])),
                part => NOP(Operand::new(part)),
            };
            Instruction { operation, output }
        })
        .collect();

    // Part 1
    let mut circuit = Circuit::new();
    for instruction in &instructions {
        circuit.run(instruction);
    }
    println!("Part 1: {}", circuit.get("a").unwrap());

    // Part 2
    let override_b = circuit.get("a").unwrap();
    let mut circuit = Circuit::new();
    circuit.circuit.insert("b", Ok(override_b));
    for instruction in &instructions {
        if instruction.output == "b" {
            continue;
        }
        circuit.run(instruction);
    }
    println!("Part 2: {}", circuit.get("a").unwrap());
}
