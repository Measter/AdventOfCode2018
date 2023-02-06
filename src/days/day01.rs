use std::{collections::HashSet, num::ParseIntError};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 1,
    name: "Chronal Calibration",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(str::trim).map(str::parse).collect()
}

fn part1(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn part2(values: &[i32]) -> i32 {
    let mut seen = HashSet::new();

    values
        .iter()
        .cycle()
        .scan(0, |state, &change| {
            *state += change;
            Some(*state)
        })
        .find(|&f| !seen.insert(f))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        assert_eq!(data.len(), 0);
    }
}
