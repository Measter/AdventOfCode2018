use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 7,
    name: "The Sum of Its Parts",
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
    b.bench(|| Ok::<_, NoError>(part2::<5, 60>(&data).0))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone)]
struct Step {
    id: u8,
    valid: bool,
    needs: [bool; 26],
}

impl Step {
    fn name(&self) -> char {
        self.id as char
    }
}

fn parse(input: &str) -> Result<Vec<Step>> {
    let mut steps: Vec<_> = (b'A'..=b'Z')
        .map(|id| Step {
            id,
            valid: false,
            needs: [false; 26],
        })
        .collect();

    for line in input.lines() {
        let line = line.trim().as_bytes();
        let needs_id = (line["Step ".len()..][0] - b'A') as usize;
        let id = (line["Step _ must be finished before step ".len()..][0] - b'A') as usize;
        let step = &mut steps[id];
        step.needs[needs_id] = true;
        step.valid = true;
        steps[needs_id].valid = true;
    }

    Ok(steps)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Waiting,
    InProgress,
    Done,
}

fn can_progress(cur_step: &Step, done_steps: &[State; 26]) -> bool {
    cur_step
        .needs
        .iter()
        .enumerate()
        .filter(|(_, n)| **n)
        .fold(true, |acc, (id, _)| (done_steps[id] == State::Done) & acc)
}

fn part1(steps: &[Step]) -> String {
    let mut done_steps = [State::Waiting; 26];
    let mut output = String::new();

    'outer: loop {
        for (i, step) in steps.iter().enumerate() {
            if !step.valid {
                continue;
            }
            if done_steps[i] == State::Waiting && can_progress(step, &done_steps) {
                output.push(step.name());
                done_steps[i] = State::Done;
                continue 'outer;
            }
        }

        break;
    }

    output
}

fn insert_worker<const N: usize>(
    workers: &mut [Option<(u8, u16)>; N],
    step_id: u8,
    step_time: u16,
) {
    if workers[0].is_none() {
        workers[0] = Some((step_id, step_time));
    } else {
        let insert_id = workers
            .iter()
            .position(|w| {
                let Some((cur_id, cur_time)) = w else {
                    return true;
                };
                step_time.cmp(cur_time).then(step_id.cmp(cur_id)).is_lt()
            })
            .unwrap();
        workers[insert_id..].rotate_right(1);
        workers[insert_id] = Some((step_id, step_time));
    }
}

fn pop_worker<const N: usize>(workers: &mut [Option<(u8, u16)>; N]) -> (u8, u16) {
    let (next_id, next_time) = workers[0].take().unwrap();
    workers.rotate_left(1);

    let mut worker_iter = workers.iter_mut();
    while let Some(Some((_, time))) = worker_iter.next() {
        *time -= next_time;
    }

    (next_id, next_time)
}

fn part2<const N: usize, const OFFSET: u16>(steps: &[Step]) -> (u16, String) {
    let mut step_state = [State::Waiting; 26];
    let mut workers: [Option<(u8, u16)>; N] = [None; N];
    let mut output = String::new();
    let mut total_time = 0;

    loop {
        for (i, step) in steps.iter().enumerate() {
            if !step.valid {
                continue;
            }
            if workers[N - 1].is_some() {
                // Out of workers, no point looking further.
                break;
            }
            if step_state[i] == State::Waiting && can_progress(step, &step_state) {
                step_state[i] = State::InProgress;
                insert_worker(&mut workers, i as u8, OFFSET + i as u16 + 1);
            }
        }

        if workers[0].is_none() {
            // We must have finished, so stop.
            break;
        }

        let (step_id, work_time) = pop_worker(&mut workers);
        total_time += work_time;
        let step_id = step_id as usize;
        output.push(steps[step_id].name());
        step_state[step_id] = State::Done;
    }

    (total_time, output)
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
        let expected = "CABDFE";
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
        let expected_str = "CABFDE";
        let expected_time = 15;
        let (actual_time, actual_str) = part2::<2, 0>(&parsed);

        assert_eq!(expected_str, actual_str);
        assert_eq!(expected_time, actual_time);
    }
}
