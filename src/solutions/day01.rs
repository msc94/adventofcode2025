use anyhow::anyhow;
use regex::Regex;

use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let line_regex = Regex::new(r"(L|R)(\d+)")?;

        let mut hit_zero = 0;
        let mut current = 50;

        for line in input.lines() {
            let captures = line_regex
                .captures(line)
                .ok_or(anyhow!("Could not match regex"))?;

            let direction = &captures[1];

            let number = captures[2].parse::<i32>()?;
            let number = if direction == "L" { -number } else { number };

            current = current + number;
            current = current % 100;
            current = if current < 0 { current + 100 } else { current };

            if current == 0 {
                hit_zero += 1;
            }
        }

        return Ok(hit_zero.to_string());
    }

    fn part2(&self, _input: &str) -> anyhow::Result<String> {
        todo!("Implement part 2")
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let input = indoc! { "
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};
        let result = Day01.part1(input)?;
        assert_eq!(result, "3");
        Ok(())
    }
}
