use crate::solutions::Solution;

pub struct Day12;

impl Solution for Day12 {
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
    fn test_part1() -> anyhow::Result<()> {
        let input = std::fs::read_to_string("inputs/day_12.txt")?;
        let result = Day12.part1(&input)?;
        assert_eq!(result, "");
        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let input = std::fs::read_to_string("inputs/day_12.txt")?;
        let result = Day12.part2(&input)?;
        assert_eq!(result, "");
        Ok(())
    }
}