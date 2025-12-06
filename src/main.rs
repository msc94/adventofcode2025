mod solutions;

use std::fs;
use std::env;
use solutions::Solution;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day> [part] [--example]", args[0]);
        eprintln!("Examples:");
        eprintln!("  {} 1              # Run day 1 with actual input", args[0]);
        eprintln!("  {} 1 1            # Run day 1 part 1 with actual input", args[0]);
        eprintln!("  {} 1 --example    # Run day 1 with example input", args[0]);
        eprintln!("  {} 1 2 --example  # Run day 1 part 2 with example input", args[0]);
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().context("Invalid day number")?;
    let mut part = "both";
    let mut use_example = false;

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "--example" => use_example = true,
            "1" | "2" => part = arg,
            _ => eprintln!("Warning: Unknown argument '{}'", arg),
        }
    }

    if day != 1 {
        anyhow::bail!("Only day 1 is currently implemented");
    }

    let input_dir = if use_example { "examples" } else { "inputs" };
    let input_file = format!("{}/day{:02}.txt", input_dir, day);
    let input = fs::read_to_string(&input_file)
        .with_context(|| format!("Could not read input file: {}", input_file))?;

    match day {
        1 => run_solution(&solutions::day01::Day01, input, part)?,
        _ => anyhow::bail!("Day {} not implemented", day),
    }

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
