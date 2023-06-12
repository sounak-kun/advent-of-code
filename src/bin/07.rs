use std::io::{self, prelude::*};

struct Instruction {
    operation: Operator,
    output: usize,
}

enum Operator {
    NOP(Operand),
    AND(Operand, Operand),
    OR(Operand, Operand),
    LSHIFT(Operand, Operand),
    RSHIFT(Operand, Operand),
    NOT(Operand),
}

enum Operand {
    Wire(usize),
    Value(u16),
}

impl Operand {
    fn new(input: &str) -> Self {
        input.parse().map_or_else(
            |_| Operand::Wire(Circuit::wire_index(input)),
            Operand::Value,
        )
    }
}

type WireState<'a> = Option<Result<u16, &'a Instruction>>;

struct Circuit<'a> {
    circuit: Vec<WireState<'a>>,
}

impl<'a> Circuit<'a> {
    fn new() -> Self {
        Self {
            // 26 * 26 (2 letter wires) + 26 (1 letter wires)
            circuit: vec![None; 26 * 26 + 26],
        }
    }

    fn wire_index(wire: &str) -> usize {
        debug_assert!(wire.len() <= 2, "{}", wire);
        wire.bytes()
            .fold(0, |acc, byte| (acc * 26) + (byte - b'a' + 1) as usize)
    }

    fn run(&mut self, instruction: &'a Instruction) {
        match &instruction.operation {
            Operator::NOP(a) | Operator::NOT(a) => {
                let a = match a {
                    Operand::Wire(wire) => self.get(*wire),
                    Operand::Value(value) => Some(*value),
                };
                let Some(a) = a else {
                    self.circuit[instruction.output] = Some(Err(instruction));
                    return;
                };
                self.circuit[instruction.output] = match instruction.operation {
                    Operator::NOP(_) => Some(Ok(a)),
                    Operator::NOT(_) => Some(Ok(!a)),
                    _ => unreachable!(),
                };
            }
            Operator::AND(a, b)
            | Operator::OR(a, b)
            | Operator::LSHIFT(a, b)
            | Operator::RSHIFT(a, b) => {
                let a = match a {
                    Operand::Wire(wire) => self.get(*wire),
                    Operand::Value(value) => Some(*value),
                };
                let Some(a) = a else {
                    self.circuit[instruction.output] = Some(Err(instruction));
                    return;
                };
                let b = match b {
                    Operand::Wire(wire) => self.get(*wire),
                    Operand::Value(value) => Some(*value),
                };
                let Some(b) = b else {
                    self.circuit[instruction.output] = Some(Err(instruction));
                    return;
                };
                self.circuit[instruction.output] = match instruction.operation {
                    Operator::AND(_, _) => Some(Ok(a & b)),
                    Operator::OR(_, _) => Some(Ok(a | b)),
                    Operator::LSHIFT(_, _) => Some(Ok(a << b)),
                    Operator::RSHIFT(_, _) => Some(Ok(a >> b)),
                    _ => unreachable!(),
                };
            }
        }
    }

    fn get(&mut self, wire: usize) -> Option<u16> {
        match self.circuit[wire]? {
            Ok(value) => Some(value),
            Err(instruction) => {
                self.run(instruction);
                self.circuit[wire]?.ok()
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
            let output = Circuit::wire_index(parts[1]);
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
    let wire_a = Circuit::wire_index("a");
    println!("Part 1: {}", circuit.get(wire_a).unwrap());

    // Part 2
    let wire_b = Circuit::wire_index("b");
    let override_b = circuit.get(wire_a).unwrap();
    let mut circuit = Circuit::new();
    circuit.circuit[wire_b] = Some(Ok(override_b));
    for instruction in &instructions {
        if instruction.output == wire_b {
            continue;
        }
        circuit.run(instruction);
    }
    println!("Part 2: {}", circuit.get(wire_a).unwrap());
}
