use std::ops::Range;

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use chumsky::{extra::Default, primitive::just, text::int, Parser};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 3,
    name: "No Matter How You Slice It",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Rectangle {
    fn parse(input: &str) -> Result<Rectangle> {
        let number = int::<_, _, Default>(10).from_str::<u16>().unwrapped();
        let id = just('#').ignore_then(number);
        let xy_pair = number.then_ignore(just(',')).then(number);
        let size_pair = number.then_ignore(just('x')).then(number);

        let line = id
            .then_ignore(just('@').padded())
            .then(xy_pair)
            .then_ignore(just(':').padded())
            .then(size_pair);

        let ((id, (x, y)), (width, height)) = line
            .parse(input)
            .into_result()
            .map_err(|e| eyre!("Error(s) parsing claim: {e:?}"))?;

        Ok(Rectangle {
            id,
            x,
            y,
            width,
            height,
        })
    }

    fn x_range(&self) -> Range<u16> {
        self.x..self.x + self.width
    }

    fn y_range(&self) -> Range<u16> {
        self.y..self.y + self.height
    }
}

fn parse(input: &str) -> Result<Vec<Rectangle>> {
    input.lines().map(str::trim).map(Rectangle::parse).collect()
}

fn part1(claims: &[Rectangle]) -> usize {
    let mut fabric = vec![0; 1000 * 1000];

    for claim in claims {
        for y in claim.y_range() {
            let start = y as usize * 1000 + claim.x as usize;
            let range = start..start + claim.width as usize;
            fabric[range].iter_mut().for_each(|i| *i += 1);
        }
    }

    fabric.into_iter().filter(|&i| i >= 2).count()
}

fn part2(claims: &[Rectangle]) -> u16 {
    let mut overlaps = vec![false; claims.len()];
    for (i, c1) in claims.iter().enumerate() {
        for (j, c2) in claims[i + 1..].iter().enumerate() {
            let x_overlap = c1.x_range().contains(&c2.x) | c2.x_range().contains(&c1.x);
            let y_overlap = c1.y_range().contains(&c2.y) | c2.y_range().contains(&c1.y);
            if x_overlap & y_overlap {
                overlaps[i] = true;
                overlaps[i + j + 1] = true;
            }
        }
    }

    let idx = overlaps.iter().position(|&i| !i).unwrap();
    claims[idx].id
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
        let expected = 4;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 3;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
