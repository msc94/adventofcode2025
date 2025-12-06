use crate::solutions::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, _input: &str) -> anyhow::Result<String> {
        todo!("Implement part 1")
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
        assert_eq!(result, "expected answer");
        Ok(())
    }
}
