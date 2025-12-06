mod solutions;

use std::fs;
use clap::{Parser, ValueEnum};
use solutions::Solution;
use anyhow::Context;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Part {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
}

#[derive(Parser, Debug)]
#[command(name = "AoC 2025")]
#[command(about = "Advent of Code 2025 solver", long_about = None)]
struct Args {
    /// Day number (1)
    day: u32,

    /// Part to run (1 or 2)
    #[arg(value_enum)]
    part: Option<Part>,

    /// Use example input instead of actual input
    #[arg(short, long)]
    example: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.day != 1 {
        anyhow::bail!("Only day 1 is currently implemented");
    }

    let input_dir = if args.example { "examples" } else { "inputs" };
    let input_file = format!("{}/day{:02}.txt", input_dir, args.day);
    let input = fs::read_to_string(&input_file)
        .with_context(|| format!("Could not read input file: {}", input_file))?;

    let part_str = match args.part {
        Some(Part::One) => "1",
        Some(Part::Two) => "2",
        None => "both",
    };

    run_solution(&solutions::day01::Day01, input, part_str)?;

    Ok(())
}

fn run_solution(solution: &dyn Solution, input: String, part: &str) -> anyhow::Result<()> {
    match part {
        "1" => println!("Part 1: {}", solution.part1(&input)?),
        "2" => println!("Part 2: {}", solution.part2(&input)?),
        "both" => {
            println!("Part 1: {}", solution.part1(&input)?);
            println!("Part 2: {}", solution.part2(&input)?);
        }
        _ => anyhow::bail!("Invalid part: {}. Use 1, 2, or 'both'", part),
    }
    Ok(())
}
