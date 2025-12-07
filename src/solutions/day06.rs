use regex::Regex;

use crate::solutions::Solution;

pub struct Day06;

#[derive(Debug, Default, Clone)]
struct Column {
    numbers: Vec<i64>,
    operation: char,
}

#[derive(Debug, Default)]
struct Input {
    columns: Vec<Column>,
}

fn parse_input(input: &str) -> anyhow::Result<Input> {
    let mut result = Input::default();

    let number_regex = Regex::new(r"\s?\d+\s?")?;
    let operation_regex = Regex::new(r"^(?:([+*])\s+)+$")?;

    for line in input.lines() {
        for (i, x) in number_regex.find_iter(line).enumerate() {
            result.columns.resize(i + 1, Column::default());
            result.columns[i].numbers.push(x.as_str().parse::<i64>()?);
        }
    }

    Ok(result)
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let input = parse_input(input)?;
        todo!()
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
        123 328  51 64 
         45 64  387 23 
          6 98  215 314
        *   +   *   +
    "};

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let result = Day06.part1(INPUT)?;
        assert_eq!(result, "");
        Ok(())
    }

    #[test]
    fn test_part2_example() -> anyhow::Result<()> {
        let result = Day06.part2(INPUT)?;
        assert_eq!(result, "");
        Ok(())
    }
}

