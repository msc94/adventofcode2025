use crate::solutions::Solution;

pub struct Day06;

impl Solution for Day06 {
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

    static INPUT: &str = indoc! {"
    "};

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let result = Day06.part1(INPUT)?;
        assert_eq!(result, "");
        Ok(())
    }

    #[test]
    fn test_part2_example() -> anmhy::Result<()> {
        let result = Day06.part2(INPUT)?;
        assert_eq!(result, "");
        Ok(())
    }
}