use crate::solutions::Solution;
use anyhow::anyhow;

pub struct Day03;

pub fn get_max_digit(str: &str) -> anyhow::Result<(char, usize)> {
    let (mut max_i, mut max_c) = (0, '0');
    for (i, c) in str.chars().enumerate() {
        if c > max_c {
            (max_i, max_c) = (i, c);
        }
    }
    return Ok((max_c, max_i));
}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let mut total = 0;

        for line in input.lines() {
            let (first_digit, first_index) = get_max_digit(&line[..line.len() - 1])?;
            let (second_digit, _) = get_max_digit(&line[first_index + 1..line.len()])?;
            let maximum = first_digit.to_string() + &second_digit.to_string();
            println!("Line: {}, First: {}, Second: {}, Max: {}", line, first_digit, second_digit, maximum);
            total += maximum.parse::<i32>()?;
        }

        Ok(total.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static INPUT: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let result = Day03.part1(INPUT)?;
        assert_eq!(result, "357");
        Ok(())
    }

    #[test]
    fn test_part2_example() -> anyhow::Result<()> {
        let result = Day03.part2(INPUT)?;
        assert_eq!(result, "4174379265");
        Ok(())
    }
}
