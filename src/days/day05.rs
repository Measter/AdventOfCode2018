use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 5,
    name: "Alchemical Reduction",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<&str> {
    Ok(input.trim())
}

fn part1(data: &str) -> usize {
    let mut bytes = data.as_bytes().to_owned();
    react(&mut bytes);
    bytes.len()
}

fn part2(data: &str) -> usize {
    (b'a'..=b'z')
        .map(|c| {
            let mut new_bytes = data.as_bytes().to_owned();
            new_bytes.retain(|&t| t.to_ascii_lowercase() != c);
            react(&mut new_bytes);
            new_bytes.len()
        })
        .min()
        .unwrap()
}

fn react(bytes: &mut Vec<u8>) {
    loop {
        if bytes.len() == 1 {
            if bytes[0] == 0 {
                bytes.clear();
            }
            break;
        }

        let mut did_modify = false;
        let mut i = 1;
        while i < bytes.len() {
            let mut a_idx = i - 1;

            while bytes[a_idx].to_ascii_lowercase() == bytes[i].to_ascii_lowercase()
                && bytes[a_idx].is_ascii_lowercase() != bytes[i].is_ascii_lowercase()
            {
                did_modify = true;

                bytes[a_idx] = 0;
                bytes[i] = 0;
                a_idx = match a_idx.checked_sub(1) {
                    Some(a_idx) => a_idx,
                    None => break,
                };
                i += 1;
                if i >= bytes.len() {
                    break;
                }
            }

            i += 1;
        }

        if did_modify {
            bytes.retain(|&b| b != 0);
        } else {
            break;
        }
    }
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
        let expected = 10;
        let actual = part1(parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 4;
        let actual = part2(parsed);

        assert_eq!(expected, actual);
    }
}
