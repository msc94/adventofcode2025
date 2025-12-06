use anyhow::anyhow;
use crate::solutions::Solution;

pub struct Day02;

fn is_repeated(n: u64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let mid = s.len() / 2;
    s[..mid] == s[mid..]
}

fn is_repeated_n(n: u64) -> bool {
    let s = n.to_string();
    for i in 1..=(s.len() / 2) {
        let pattern = &s[..i];
        if pattern.repeat(s.len() / pattern.len()) == s {
            return true;
        }
    }
    false
}

fn parse_range(range: &str) -> anyhow::Result<(u64, u64)> {
    let mut parts = range.split('-');
    let begin = parts
        .next()
        .ok_or(anyhow!("Invalid range"))?
        .trim()
        .parse::<u64>()?;
    let end = parts
        .next()
        .ok_or(anyhow!("Invalid range"))?
        .trim()
        .parse::<u64>()?;
    Ok((begin, end))
}

fn sum_matching<F>(input: &str, predicate: F) -> anyhow::Result<u64>
where
    F: Fn(u64) -> bool,
{
    let mut result = 0u64;
    for range in input.split(',') {
        let (begin, end) = parse_range(range)?;
        for n in begin..=end {
            if predicate(n) {
                result += n;
            }
        }
    }
    Ok(result)
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        sum_matching(input, is_repeated).map(|n| n.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        sum_matching(input, is_repeated_n).map(|n| n.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let result = Day02.part1(INPUT)?;
        assert_eq!(result, "1227775554");
        Ok(())
    }

    #[test]
    fn test_part2_example() -> anyhow::Result<()> {
        let result = Day02.part2(INPUT)?;
        assert_eq!(result, "4174379265");
        Ok(())
    }
}
