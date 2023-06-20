use std::io::{self, prelude::*};

const GRID_SIZE: usize = 100;

#[derive(Clone)]
struct LightGrid {
    grid: [[[u8; GRID_SIZE + 2]; GRID_SIZE + 2]; 2],
    swap: bool,
}

impl Default for LightGrid {
    fn default() -> Self {
        LightGrid {
            grid: [[[0; GRID_SIZE + 2]; GRID_SIZE + 2]; 2],
            swap: false,
        }
    }
}

impl LightGrid {
    fn step(&mut self) {
        // Determine which grid to read from and write to, then swap
        let (src, dst) = if self.swap { (1, 0) } else { (0, 1) };
        self.swap = !self.swap;

        // Step through the grid
        const NEIGHBORS: [(isize, isize); 8] = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        for y in 1..=GRID_SIZE {
            for x in 1..=GRID_SIZE {
                self.grid[dst][y][x] = match NEIGHBORS
                    .iter()
                    .map(|&(dx, dy)| {
                        self.grid[src][(y as isize + dy) as usize][(x as isize + dx) as usize]
                    })
                    .sum()
                {
                    3 => 1,
                    2 => self.grid[src][y][x],
                    _ => 0,
                }
            }
        }
    }

    fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    fn current(&mut self) -> &mut [[u8; GRID_SIZE + 2]; GRID_SIZE + 2] {
        match self.swap {
            true => &mut self.grid[1],
            false => &mut self.grid[0],
        }
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse input
    let mut grid = LightGrid::default();
    input.trim().lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, b)| match b {
            b'#' => grid.grid[0][y + 1][x + 1] = 1,
            _ => (),
        });
    });

    // Part 1
    let mut one = grid.clone();
    one.run(100);
    println!(
        "Part 1: {}",
        one.current()
            .iter()
            .flatten()
            .fold(0, |sum, &b| sum + b as usize)
    );

    // Part 2
    let mut two = grid.clone();
    two.current()[1][1] = 1;
    two.current()[1][GRID_SIZE] = 1;
    two.current()[GRID_SIZE][1] = 1;
    two.current()[GRID_SIZE][GRID_SIZE] = 1;
    for _ in 0..100 {
        two.step();
        two.current()[1][1] = 1;
        two.current()[1][GRID_SIZE] = 1;
        two.current()[GRID_SIZE][1] = 1;
        two.current()[GRID_SIZE][GRID_SIZE] = 1;
    }
    println!(
        "Part 2: {}",
        two.current()
            .iter()
            .flatten()
            .fold(0, |sum, &b| sum + b as usize)
    );
}
