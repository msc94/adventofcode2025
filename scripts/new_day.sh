#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <day>"
    echo "Example: $0 5"
    exit 1
fi

DAY=$1
DAY_PADDED=$(printf "%02d" "$DAY")
DAY_MOD="day$(printf "%02d" "$DAY")"
DAY_STRUCT="Day$DAY_PADDED"

# Verify day is valid
if [ "$DAY" -lt 1 ] || [ "$DAY" -gt 25 ]; then
    echo "Error: Day must be between 1 and 25"
    exit 1
fi

# Check if day already exists
if [ -f "src/solutions/${DAY_MOD}.rs" ]; then
    echo "Error: $DAY_MOD already exists"
    exit 1
fi

# Create solution file
cat > "src/solutions/${DAY_MOD}.rs" << DAYEOF
use crate::solutions::Solution;

pub struct $DAY_STRUCT;

impl Solution for $DAY_STRUCT {
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
        let result = $DAY_STRUCT.part1(input).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_part2_example() {
        let input = "";
        let result = $DAY_STRUCT.part2(input).unwrap();
        assert_eq!(result, "");
    }
}
DAYEOF

# Create empty input files
touch "inputs/day${DAY_PADDED}.txt"
touch "examples/day${DAY_PADDED}.txt"

# Update mod.rs - add module declaration if not present
if ! grep -q "pub mod ${DAY_MOD};" src/solutions/mod.rs; then
    # Insert after the last pub mod line
    sed -i "/^pub mod day/\$ a pub mod ${DAY_MOD};" src/solutions/mod.rs
fi

# Update mod.rs - add to get_solution match if not present
if ! grep -q "$DAY =>" src/solutions/mod.rs; then
    # Insert before the _ => line
    sed -i "/^        _ => anyhow::bail!/i\        $DAY => Ok(Box::new(${DAY_MOD}::${DAY_STRUCT}))," src/solutions/mod.rs
fi

echo "✓ Created scaffolding for Day $DAY:"
echo "  - src/solutions/${DAY_MOD}.rs"
echo "  - inputs/day${DAY_PADDED}.txt"
echo "  - examples/day${DAY_PADDED}.txt"
echo ""
echo "Next steps:"
echo "  1. Add your input to inputs/day${DAY_PADDED}.txt"
echo "  2. Add example input to examples/day${DAY_PADDED}.txt"
echo "  3. Implement part1() and part2() in src/solutions/${DAY_MOD}.rs"
echo "  4. Test with: cargo test solutions::${DAY_MOD}"
echo "  5. Run with: cargo run --release -- --day $DAY"
