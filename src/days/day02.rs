use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};
use itertools::Itertools;

pub const DAY: Day = Day {
    day: 2,
    name: "Inventory Management System",
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

fn parse(input: &str) -> Result<Vec<&str>> {
    let mut ids = Vec::new();

    for line in input.lines().map(str::trim) {
        if line.bytes().any(|b| !b.is_ascii_alphabetic()) {
            return Err(eyre!("Invalid character in ID: `{line}`"));
        }

        ids.push(line);
    }

    Ok(ids)
}

fn part1(ids: &[&str]) -> u16 {
    let mut num_threes = 0;
    let mut num_twos = 0;

    for id in ids {
        let counts = id.bytes().fold([0u8; 26], |mut acc, b| {
            acc[(b - b'a') as usize] += 1;
            acc
        });

        let (has_two, has_three) = counts
            .into_iter()
            .fold((false, false), |(has_two, has_three), count| {
                (has_two || count == 2, has_three || count == 3)
            });

        num_threes += has_three as u16;
        num_twos += has_two as u16;
    }

    num_threes * num_twos
}

fn part2(ids: &[&str]) -> String {
    let Some((a, b)) = ids.iter()
        .cartesian_product(ids)
        .find(|(a, b)| a.bytes().zip(b.bytes()).filter(|(a, b)| a != b).count() == 1) else {
            panic!("Matched pair not found");
        };

    a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
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

        let parsed = parse(&data).unwrap();
        let expected = 12;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = "fgij";
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
