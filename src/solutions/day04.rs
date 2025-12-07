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

fn get_surrounds_vec(lines: &Vec<Vec<u8>>, x: i64, y: i64) -> Vec<u8> {
    let slice = lines.iter().map(|x| x.as_slice()).collect::<Vec<&[u8]>>();
    get_surrounds(&slice, x, y)
}

fn print_field(lines: &Vec<Vec<u8>>) {
    for line in lines {
        println!("{}", String::from_utf8_lossy(line));
    }
}

fn remove_all(lines: &Vec<Vec<u8>>) -> (Vec<Vec<u8>>, usize) {
    let mut result = lines.clone();
    let mut removed = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] != b'@' {
                continue;
            }

            let surrounds = get_surrounds_vec(lines, x as i64, y as i64)
                .iter()
                .filter(|c| **c == b'@')
                .count();

            if surrounds < 4 {
                result[y][x] = b'.';
                removed += 1;
            }
        }
    }

    (result, removed)
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

    fn part2(&self, input: &str) -> anyhow::Result<String> {
        let mut lines = input
            .lines()
            .map(|x| x.as_bytes().iter().cloned().collect())
            .collect::<Vec<Vec<u8>>>();

        let mut total_removed = 0;

        loop {
            let (new_lines, removed) = remove_all(&lines);

            if removed == 0 {
                break;
            }

            lines = new_lines;
            total_removed += removed;
        }

        Ok(total_removed.to_string())
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
    fn test_part2_example() -> anyhow::Result<()> {
        let result = Day04.part2(INPUT)?;
        assert_eq!(result, "43");
        Ok(())
    }
}
