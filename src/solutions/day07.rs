use crate::solutions::Solution;
use anyhow::Result;

pub struct Day07;

type Field = Vec<Vec<u8>>;

fn parse(str: &str) -> anyhow::Result<Field> {
    Ok(str.lines().map(|line| line.as_bytes().to_vec()).collect())
}

#[allow(dead_code)]
fn field_to_str(field: &Field) -> String {
    field
        .iter()
        .map(|row| String::from_utf8_lossy(row).to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_field_entry(field: &Field, x: usize, y: usize) -> Option<u8> {
    if y >= field.len() || x >= field[y].len() {
        None
    } else {
        Some(field[y][x])
    }
}

fn process(field: &Field) -> Result<(Field, bool, u64)> {
    let mut one_changed = false;
    let mut updated = field.clone();
    let mut mirrored = 0;

    for y in 0..field.len() {
        for x in 0..field[y].len() {
            let entry = get_field_entry(field, x, y).ok_or(anyhow::anyhow!("Out of bounds"))?;

            if entry != b'|' && entry != b'S' {
                continue;
            }

            if let Some(below) = get_field_entry(field, x, y + 1) {
                if below == b'.' {
                    updated[y + 1][x] = b'|';
                    one_changed = true;
                } else if below == b'^' {
                    let mut one_updated = false;
                    if let Some(b'.') = get_field_entry(field, x - 1, y + 1) {
                        updated[y + 1][x - 1] = b'|';
                        one_updated = true;
                    }
                    if let Some(b'.') = get_field_entry(field, x + 1, y + 1) {
                        updated[y + 1][x + 1] = b'|';
                        one_updated = true;
                    }
                    if one_updated {
                        mirrored += 1;
                        one_changed = true;
                    }
                }
            }
        }
    }

    Ok((updated, one_changed, mirrored))
}

impl Solution for Day07 {
    fn part1(&self, input: &str) -> anyhow::Result<String> {
        let mut field = parse(input)?;
        let mut result = 0;
        loop {
            let (updated, changed, mirrored) = process(&field)?;
            if !changed {
                break;
            }
            field = updated;
            result += mirrored;
            // println!("{}", field_to_str(&field));
            // println!()
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
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn test_part1_example() -> anyhow::Result<()> {
        let result = Day07.part1(INPUT)?;
        assert_eq!(result, "21");
        Ok(())
    }

    #[ignore]
    #[test]
    fn test_part2_example() -> anyhow::Result<()> {
        let result = Day07.part2(INPUT)?;
        assert_eq!(result, "");
        Ok(())
    }
}

