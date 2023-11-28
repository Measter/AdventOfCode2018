use std::{collections::HashMap, ops::Range};

use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use chumsky::{primitive::just, text::digits, Parser as _};
use color_eyre::{eyre::eyre, Report, Result};

use crate::Parser;

pub const DAY: Day = Day {
    day: 4,
    name: "Repose Record",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TimeStamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl TimeStamp {
    fn parser<'a>() -> impl Parser<'a, TimeStamp> {
        let year_num = digits(10).to_slice().from_str::<u16>().unwrapped();
        let other_num = digits(10).to_slice().from_str::<u8>().unwrapped();

        let date = year_num
            .then_ignore(just('-'))
            .then(other_num)
            .then_ignore(just('-'))
            .then(other_num);
        let time = other_num.then_ignore(just(':')).then(other_num);

        date.then_ignore(just(' '))
            .then(time)
            .map(|(((year, month), day), (hour, minute))| TimeStamp {
                year,
                month,
                day,
                hour,
                minute,
            })
    }
}

#[derive(Debug, Clone, Copy)]
enum EventKind {
    OnDuty(u16),
    Sleep,
    Wake,
}

impl EventKind {
    fn parser<'a>() -> impl Parser<'a, EventKind> {
        let number = digits(10).to_slice().from_str::<u16>().unwrapped();
        let id = just('#').ignore_then(number);

        let on_duty = just("Guard")
            .ignore_then(id.padded())
            .then_ignore(just("begins shift"))
            .map(EventKind::OnDuty);

        let sleep = just("falls asleep").to(EventKind::Sleep);
        let wake = just("wakes up").to(EventKind::Wake);

        on_duty.or(sleep).or(wake)
    }
}

#[derive(Debug, Clone)]
struct Event {
    timestamp: TimeStamp,
    kind: EventKind,
}

impl Event {
    fn parse(line: &str) -> Result<Event> {
        let parser = TimeStamp::parser()
            .delimited_by(just('['), just(']'))
            .then_ignore(just(' '))
            .then(EventKind::parser())
            .map(|(timestamp, kind)| Event { timestamp, kind });

        let res = parser
            .parse(line)
            .into_result()
            .map_err(|e| eyre!("Failed to parse: {e:?}"))?;

        Ok(res)
    }
}

fn parse(input: &str) -> Result<Vec<Event>> {
    input.lines().map(str::trim).map(Event::parse).collect()
}

fn part1(events: &[Event]) -> u32 {
    let mut events = events.to_owned();
    events.sort_unstable_by_key(|e| e.timestamp);

    let mut sleep_ranges = HashMap::new();
    let mut cur_id = 0;
    let mut cur_sleep_start = 0;
    for event in &events {
        match event.kind {
            EventKind::OnDuty(id) => cur_id = id,
            EventKind::Sleep => cur_sleep_start = event.timestamp.minute,
            EventKind::Wake => {
                let total: &mut Vec<Range<u8>> = sleep_ranges.entry(cur_id).or_default();
                total.push(cur_sleep_start..event.timestamp.minute);
            }
        }
    }

    let (&sleepiest, ranges) = sleep_ranges
        .iter()
        .max_by_key(|(_, ranges)| ranges.iter().map(|r| r.clone().count()).sum::<usize>())
        .unwrap();

    let mut minutes = [0_u16; 60];

    for range in ranges {
        for i in range.clone() {
            minutes[i as usize] += 1;
        }
    }

    let (minute, _) = minutes.iter().enumerate().max_by_key(|(_, &a)| a).unwrap();

    sleepiest as u32 * minute as u32
}

fn part2(events: &[Event]) -> u32 {
    let mut events = events.to_owned();
    events.sort_unstable_by_key(|e| e.timestamp);

    let mut sleep_ranges = HashMap::new();
    let mut cur_id = 0;
    let mut cur_sleep_start = 0;
    for event in &events {
        match event.kind {
            EventKind::OnDuty(id) => cur_id = id,
            EventKind::Sleep => cur_sleep_start = event.timestamp.minute,
            EventKind::Wake => {
                let total: &mut Vec<Range<u8>> = sleep_ranges.entry(cur_id).or_default();
                total.push(cur_sleep_start..event.timestamp.minute);
            }
        }
    }

    let (id, minute, _) = sleep_ranges
        .iter()
        .map(|(id, ranges)| {
            let mut minutes = [0_u16; 60];

            for range in ranges {
                for i in range.clone() {
                    minutes[i as usize] += 1;
                }
            }

            let (minute, times_slept) = minutes.iter().enumerate().max_by_key(|(_, &a)| a).unwrap();
            (*id, minute, *times_slept)
        })
        .max_by_key(|(_, _, ts)| *ts)
        .unwrap();

    id as u32 * minute as u32
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
        let expected = 240;
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
        let expected = 4455;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
