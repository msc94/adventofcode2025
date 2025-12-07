use crate::solutions::Solution;

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

pub fn get_solution(str: &str, num: usize) -> anyhow::Result<u64> {
    let mut result = String::new();
    let mut str = str;

    for i in 0..num {
        let still_needed = num - i;
        let (digit, index) = get_max_digit(&str[..=str.len() - still_needed])?;
        result.push(digit);
        str = &str[index + 1..];
    }

    Ok(result.parse::<u64>()?)
}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let mut total = 0;

        for line in input.lines() {
            total += get_solution(line, 2)?;
        }

        Ok(total.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let mut total = 0;

        for line in input.lines() {
            total += get_solution(line, 12)?;
        }

        Ok(total.to_string())
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
        assert_eq!(result, "3121910778619");
        Ok(())
    }
}
