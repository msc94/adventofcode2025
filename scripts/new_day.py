#!/usr/bin/env python3
import sys
import re
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
    fn test_part1_example() {
        let input = "";
        let result = {{ day_struct }}.part1(input).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_part2_example() {
        let input = "";
        let result = {{ day_struct }}.part2(input).unwrap();
        assert_eq!(result, "");
    }
}
"""

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 new_day.py <day>")
        print("Example: python3 new_day.py 5")
        sys.exit(1)

    try:
        day = int(sys.argv[1])
    except ValueError:
        print("Error: Day must be a number")
        sys.exit(1)

    if day < 1 or day > 25:
        print("Error: Day must be between 1 and 25")
        sys.exit(1)

    day_padded = f"{day:02d}"
    day_mod = f"day{day_padded}"
    day_struct = f"Day{day_padded}"

    root = Path(__file__).parent.parent
    solution_file = root / "src" / "solutions" / f"{day_mod}.rs"
    input_file = root / "inputs" / f"{day_padded}.txt"
    example_file = root / "examples" / f"{day_padded}.txt"
    mod_file = root / "src" / "solutions" / "mod.rs"

    # Check if day already exists
    if solution_file.exists():
        print(f"Error: {day_mod} already exists")
        sys.exit(1)

    # Render solution template
    template = Template(SOLUTION_TEMPLATE)
    solution_code = template.render(day_struct=day_struct)

    # Create solution file
    solution_file.write_text(solution_code)

    # Create empty input files
    input_file.touch()
    example_file.touch()

    # Update mod.rs
    mod_content = mod_file.read_text()

    # Add module declaration if not present
    if f"pub mod {day_mod};" not in mod_content:
        # Find the last pub mod line and add after it
        lines = mod_content.split('\n')
        last_mod_idx = -1
        for i, line in enumerate(lines):
            if line.startswith('pub mod day'):
                last_mod_idx = i
        
        if last_mod_idx >= 0:
            lines.insert(last_mod_idx + 1, f"pub mod {day_mod};")
            mod_content = '\n'.join(lines)
            mod_file.write_text(mod_content)

    # Add to get_solution match if not present
    if f"{day} =>" not in mod_content:
        mod_content = re.sub(
            r'(\s+)_ => anyhow::bail!',
            f'        {day} => Ok(Box::new({day_mod}::{day_struct})),\n        _ => anyhow::bail!',
            mod_content
        )
        mod_file.write_text(mod_content)

    print(f"✓ Created scaffolding for Day {day}:")
    print(f"  - src/solutions/{day_mod}.rs")
    print(f"  - inputs/day{day_padded}.txt")
    print(f"  - examples/day{day_padded}.txt")
    print()
    print("Next steps:")
    print(f"  1. Add your input to inputs/day{day_padded}.txt")
    print(f"  2. Add example input to examples/day{day_padded}.txt")
    print(f"  3. Implement part1() and part2() in src/solutions/{day_mod}.rs")
    print(f"  4. Test with: cargo test solutions::{day_mod}")
    print(f"  5. Run with: cargo run --release -- --day {day}")

if __name__ == "__main__":
    main()
