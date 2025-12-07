use std::ops::RangeInclusive;

use anyhow::anyhow;
use regex::Regex;

use crate::solutions::Solution;

pub struct Day05;

#[derive(Debug, Default)]
struct Input {
    ranges: Vec<RangeInclusive<i64>>,
    values: Vec<i64>,
}

fn parse_input(input: &str) -> anyhow::Result<Input> {
    let mut result = Input::default();
    let mut lines = input.lines();

    // Parse ranges
    let range_regex = Regex::new(r"(\d+)-(\d+)")?;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let captures = range_regex
            .captures(line)
            .ok_or(anyhow!("Regex could not be matched"))?;

        let begin = captures[1].parse::<i64>()?;
        let end = captures[2].parse::<i64>()?;
        result.ranges.push(begin..=end);
    }

    // Parse values
    while let Some(line) = lines.next() {
        result.values.push(line.parse::<i64>()?);
    }

    Ok(result)
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let input = parse_input(input)?;
        Ok(input
            .values
            .iter()
            .filter(|v| input.ranges.iter().any(|r| r.contains(v)))
            .count()
            .to_string())
    }

    fn part2(&self, _input: &str) -> anyhow::Result<String> {
        todo!("Implement part 2")
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let result = Day05.part1(INPUT)?;
        assert_eq!(result, "");
        Ok(())
    }

    #[test]
    fn test_part2_example() -> anyhow::Result<()> {
        let result = Day05.part2(INPUT)?;
        assert_eq!(result, "");
        Ok(())
    }
}

