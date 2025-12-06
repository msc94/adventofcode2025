# Advent of Code 2025 - Rust Solution

A Rust scaffolding project for solving Advent of Code 2025 problems.

## Structure

```
.
├── src/
│   ├── main.rs          # Entry point with CLI argument handling
│   └── solutions/       # Solution modules
│       ├── mod.rs       # Modules and Solution trait
│       ├── day01.rs     # Day 1 solution
│       └── ...
├── inputs/              # Input files (day01.txt, day02.txt, etc.)
└── Cargo.toml          # Project manifest
```

## Usage

### Running a solution:

```bash
# Run both parts for day 1
cargo run -- 1

# Run only part 1 of day 5
cargo run -- 5 1

# Run only part 2 of day 5
cargo run -- 5 2
```

### Adding a solution:

1. Edit `src/solutions/dayXX.rs` and implement the `Solution` trait:

```rust
use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, _input: &str) -> String {
        "answer".to_string()
    }

    fn part2(&self, _input: &str) -> String {
        "answer".to_string()
    }
}
```

2. Place your input file at `inputs/dayXX.txt`

3. Run with `cargo run -- XX`

## Building

```bash
cargo build --release
```

The compiled binary will be at `target/release/aoc2025`
