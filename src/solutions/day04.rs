use itertools::Itertools;

use crate::solutions::Solution;

pub struct Day04;

fn get_byte(lines: &[&[u8]], x: i64, y: i64) -> Option<u8> {
    if y >= 0 && x >= 0 && y < lines.len() as i64 && x < lines[y as usize].len() as i64 {
        Some(lines[y as usize][x as usize])
    } else {
        None
    }
}

fn get_surrounds(lines: &[&[u8]], x: i64, y: i64) -> Vec<u8> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(dx, dy)| *dx != 0 || *dy != 0)
        .filter_map(|(dx, dy)| get_byte(lines, x + dx, y + dy))
        .collect()
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let lines = input.lines().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();

        let result = (0..lines.len())
            .flat_map(|y| (0..lines[y].len()).map(move |x| (x, y)))
            .filter(|(x, y)| lines[*y][*x] == b'@')
            .map(|(x, y)| get_surrounds(&lines, x as i64, y as i64))
            .filter(|s| s.iter().filter(|c| **c == b'@').count() < 4)
            .count()
            .to_string();

        Ok(result)
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
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let result = Day04.part1(INPUT)?;
        assert_eq!(result, "13");
        Ok(())
    }

    #[test]
    fn test_part2_example() {
        let result = Day04.part2(INPUT).unwrap();
        assert_eq!(result, "");
    }
}
