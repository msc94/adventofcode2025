use itertools::Itertools;

use crate::solutions::Solution;

pub struct Day04;

fn get_byte(lines: &[&[u8]], x: i64, y: i64) -> Option<u8> {
    let y_usize = usize::try_from(y).ok()?;
    let x_usize = usize::try_from(x).ok()?;
    lines.get(y_usize)?.get(x_usize).copied()
}

fn count_at_neighbors(lines: &[&[u8]], x: i64, y: i64, byte: u8) -> usize {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
        .filter_map(|(dx, dy)| get_byte(lines, x + dx, y + dy))
        .filter(|&b| b == byte)
        .count()
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let lines = input.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();

        let result = (0..lines.len())
            .flat_map(|y| (0..lines[y].len()).map(move |x| (x, y)))
            .filter(|(x, y)| lines[*y][*x] == b'@' && count_at_neighbors(&lines, *x as i64, *y as i64, b'@') < 4)
            .count();

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let mut grid: Vec<Vec<u8>> = input
            .lines()
            .map(|x| x.as_bytes().to_vec())
            .collect();

        let mut total = 0;
        loop {
            let mut removed = Vec::new();
            
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] == b'@' {
                        let lines_refs: Vec<_> = grid.iter().map(|r| r.as_slice()).collect();
                        if count_at_neighbors(&lines_refs, x as i64, y as i64, b'@') < 4 {
                            removed.push((x, y));
                        }
                    }
                }
            }

            if removed.is_empty() {
                break;
            }

            total += removed.len();
            for (x, y) in removed {
                grid[y][x] = b'.';
            }
        }

        Ok(total.to_string())
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
        assert_eq!(Day04.part1(INPUT)?, "13");
        Ok(())
    }

    #[test]
    fn test_part2_example() -> anyhow::Result<()> {
        assert_eq!(Day04.part2(INPUT)?, "43");
        Ok(())
    }
}
