pub mod day01;
pub mod day02;
pub mod day03;
pub mod day05;

pub trait Solution {
    fn part1(&self, input: &str) -> anyhow::Result<String>;
    fn part2(&self, input: &str) -> anyhow::Result<String>;
}

pub fn get_solution(day: u32) -> anyhow::Result<Box<dyn Solution>> {
    match day {
        1 => Ok(Box::new(day01::Day01)),
        2 => Ok(Box::new(day02::Day02)),
        3 => Ok(Box::new(day03::Day03)),
        5 => Ok(Box::new(day05::Day05)),
        _ => anyhow::bail!("Day {} not yet implemented", day),
    }
}
