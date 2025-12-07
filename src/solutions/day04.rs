use crate::solutions::Solution;

pub struct Day04;

pub fn get_byte(lines: &[&[u8]], x: i64, y: i64) -> Option<u8> {
    if x >= 0 && y >= 0 && x < lines.len() as i64 && y < lines[x as usize].len() as i64 {
        Some(lines[x as usize][y as usize])
    } else {
        None
    }
}

pub fn get_surrounds(lines: &[&[u8]], x: i64, y: i64) -> Vec<u8> {
    let mut surrounds = Vec::new();
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some(b) = get_byte(lines, x + dx, y + dy) {
                surrounds.push(b);
            }
        }
    }
    surrounds
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let mut result = 0;

        let lines = input.lines().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();

        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                if lines[y][x] != b'@' {
                    continue;
                }

                let surrounds = get_surrounds(&lines, y as i64, x as i64)
                    .iter()
                    .filter(|x| **x == b'@')
                    .count();

                if surrounds < 4 {
                    result += 1;
                }
            }
        }
        Ok(result.to_string())
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
