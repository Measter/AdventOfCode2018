use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};

pub const DAY: Day = Day {
    day: 9,
    name: "Marble Mania",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Clone, Copy)]
struct Game {
    players: usize,
    marbles: usize,
}

fn parse(input: &str) -> Result<Game> {
    let (players, rest) = input
        .split_once(' ')
        .ok_or_else(|| eyre!("invalid input: `{input:?}`"))?;
    let (_, rest) = rest
        .split_once("h ")
        .ok_or_else(|| eyre!("invalid input: `{input:?}`"))?;
    let (marbles, _) = rest
        .split_once(' ')
        .ok_or_else(|| eyre!("invalid input: `{input:?}`"))?;

    Ok(Game {
        players: players.parse()?,
        marbles: marbles.parse()?,
    })
}

#[derive(Debug)]
struct Playground {
    board: Vec<usize>,
    cursor: usize,
}

impl Playground {
    fn new(marbles: usize) -> Self {
        Self {
            board: {
                let mut v = Vec::with_capacity(marbles);
                v.push(0);
                v
            },
            cursor: 0,
        }
    }

    fn play(&mut self, marble: usize) -> Option<usize> {
        if marble % 23 == 0 {
            let cursor_pos = match self.cursor.checked_sub(7) {
                Some(cp) => cp,
                None => self.cursor + self.board.len() - 7,
            };

            let removed = self.board.remove(cursor_pos);
            self.cursor = cursor_pos;

            Some(removed)
        } else if self.board.len() == 1 {
            self.board.push(marble);
            self.cursor = 1;
            None
        } else {
            let insert_pos = self.cursor + 2;
            let insert_pos = if insert_pos == self.board.len() {
                self.board.push(marble);
                insert_pos
            } else {
                let insert_pos = (self.cursor + 2) % self.board.len();
                self.board.insert(insert_pos, marble);
                insert_pos
            };
            self.cursor = insert_pos;
            None
        }
    }
}

fn part1(data: Game) -> usize {
    let mut playground = Playground::new(data.marbles);
    let mut scores = vec![0; data.players];
    let player_iter = (0..scores.len()).cycle();

    for (marble, player_id) in (1..=data.marbles).zip(player_iter) {
        if let Some(removed_marble) = playground.play(marble) {
            scores[player_id] += removed_marble + marble;
        }
    }

    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn wrap_test() {
        let mut playground = Playground {
            board: vec![0, 1, 2, 3, 4, 5, 6, 7],
            cursor: 2,
        };

        let expected = Some(3);
        let actual = playground.play(23);
        assert_eq!(actual, expected);
        assert_eq!(playground.board, vec![0, 1, 2, 4, 5, 6, 7]);
        assert_eq!(playground.cursor, 3);
    }

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        for line in data.lines() {
            let (game, result) = line.split_once(':').unwrap();
            let parsed = parse(game).unwrap();
            let expected: usize = result.parse().unwrap();
            let actual = part1(parsed);
            assert_eq!(expected, actual);
            eprintln!("Passed {parsed:?}");
        }
    }
}
