mod solutions;

use std::fs;
use std::env;
use solutions::Solution;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <day> [part]", args[0]);
        eprintln!("Example: {} 1 1", args[0]);
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().expect("Invalid day number");
    let part = args.get(2).map(|p| p.as_str()).unwrap_or("both");

    if day < 1 || day > 25 {
        eprintln!("Day must be between 1 and 25");
        std::process::exit(1);
    }

    let input_file = format!("inputs/day{:02}.txt", day);
    let input = fs::read_to_string(&input_file)
        .unwrap_or_else(|_| {
            eprintln!("Could not read input file: {}", input_file);
            String::new()
        });

    match day {
        1 => run_solution(&solutions::day01::Day01, input, part),
        2 => run_solution(&solutions::day02::Day02, input, part),
        3 => run_solution(&solutions::day03::Day03, input, part),
        4 => run_solution(&solutions::day04::Day04, input, part),
        5 => run_solution(&solutions::day05::Day05, input, part),
        6 => run_solution(&solutions::day06::Day06, input, part),
        7 => run_solution(&solutions::day07::Day07, input, part),
        8 => run_solution(&solutions::day08::Day08, input, part),
        9 => run_solution(&solutions::day09::Day09, input, part),
        10 => run_solution(&solutions::day10::Day10, input, part),
        11 => run_solution(&solutions::day11::Day11, input, part),
        12 => run_solution(&solutions::day12::Day12, input, part),
        13 => run_solution(&solutions::day13::Day13, input, part),
        14 => run_solution(&solutions::day14::Day14, input, part),
        15 => run_solution(&solutions::day15::Day15, input, part),
        16 => run_solution(&solutions::day16::Day16, input, part),
        17 => run_solution(&solutions::day17::Day17, input, part),
        18 => run_solution(&solutions::day18::Day18, input, part),
        19 => run_solution(&solutions::day19::Day19, input, part),
        20 => run_solution(&solutions::day20::Day20, input, part),
        21 => run_solution(&solutions::day21::Day21, input, part),
        22 => run_solution(&solutions::day22::Day22, input, part),
        23 => run_solution(&solutions::day23::Day23, input, part),
        24 => run_solution(&solutions::day24::Day24, input, part),
        25 => run_solution(&solutions::day25::Day25, input, part),
        _ => eprintln!("Day {} not yet implemented", day),
    }
}

fn run_solution(solution: &dyn Solution, input: String, part: &str) {
    match part {
        "1" => println!("Part 1: {}", solution.part1(&input)),
        "2" => println!("Part 2: {}", solution.part2(&input)),
        "both" => {
            println!("Part 1: {}", solution.part1(&input));
            println!("Part 2: {}", solution.part2(&input));
        }
        _ => eprintln!("Invalid part: {}. Use 1, 2, or 'both'", part),
    }
}
