#!/usr/bin/env python3
import argparse
from pathlib import Path
from jinja2 import Template

SOLUTION_TEMPLATE = """use crate::solutions::Solution;

pub struct {{ day_struct }};

impl Solution for {{ day_struct }} {
    fn part1(&self, _input: &str) -> anyhow::Result<String> {
        todo!("Implement part 1")
    }

    fn part2(&self, _input: &str) -> anyhow::Result<String> {
        todo!("Implement part 2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let input = std::fs::read_to_string("inputs/day_{{ day_padded }}.txt")?;
        let result = {{ day_struct }}.part1(&input)?;
        assert_eq!(result, "");
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let input = std::fs::read_to_string("inputs/day_{{ day_padded }}.txt")?;
        let result = {{ day_struct }}.part2(&input)?;
        assert_eq!(result, "");
        Ok(())
    }
}
"""

def parse_arguments():
    parser = argparse.ArgumentParser(
        description="Create scaffolding for a new Advent of Code day",
        epilog="Example: python3 new_day.py 5"
    )
    parser.add_argument(
        "day",
        type=int,
        help="Day number (1-25)"
    )
    parser.add_argument(
        "--no-tests",
        action="store_true",
        help="Skip creating test file"
    )

    args = parser.parse_args()

    if args.day < 1 or args.day > 25:
        parser.error("Day must be between 1 and 25")

    return args

def main():
    args = parse_arguments()
    
    day = args.day
    day_padded = f"{day:02d}"
    day_mod = f"day{day_padded}"
    day_struct = f"Day{day_padded}"

    root = Path(__file__).parent.parent
    solution_file = root / "src" / "solutions" / f"{day_mod}.rs"
    input_file = root / "inputs" / f"day_{day_padded}.txt"

    # Check if day already exists
    if solution_file.exists():
        print(f"Error: {day_mod} already exists")
        return 1

    # Render solution template
    template = Template(SOLUTION_TEMPLATE)
    solution_code = template.render(day_struct=day_struct, day_padded=day_padded)

    # Create solution file
    solution_file.write_text(solution_code)

    # Create empty input file
    input_file.touch()

    print(f"✓ Created scaffolding for Day {day}:")
    print(f"  - src/solutions/{day_mod}.rs")
    print(f"  - inputs/day_{day_padded}.txt")
    
    return 0

if __name__ == "__main__":
    exit(main())
