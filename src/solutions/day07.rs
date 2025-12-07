use crate::solutions::Solution;

pub struct Day07;

impl Solution for Day07 {
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
    fn test_part1_example() {
        let input = "";
        let result = Day07.part1(input).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_part2_example() {
        let input = "";
        let result = Day07.part2(input).unwrap();
        assert_eq!(result, "");
    }
}