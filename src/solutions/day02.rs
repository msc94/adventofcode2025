use anyhow::anyhow;

use crate::solutions::Solution;

pub struct Day01;

fn is_repeated(i: i32) -> bool {
    let str = i.to_string();

    if str.len() % 2 != 0 {
        return false;
    }

    return str[..str.len() / 2] == str[str.len() / 2..];
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let mut result = 0;
        let ranges = input.split(',');
        for range in ranges {
            let mut parts = range.split('-');
            let begin = parts
                .next()
                .ok_or(anyhow!("Invalid range"))?
                .parse::<i32>()?;
            let end = parts
                .next()
                .ok_or(anyhow!("Invalid range"))?
                .parse::<i32>()?;

            for i in begin..=end {
                if is_repeated(i) {
                    result += i;
                }
            }
        }
        Ok(result.to_string())
    }

    fn part2(&self, _input: &str) -> anyhow::Result<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let result = Day01.part1(INPUT)?;
        assert_eq!(result, "1227775554");
        Ok(())
    }
}
