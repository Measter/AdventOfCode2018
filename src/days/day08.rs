use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{eyre::eyre, Report, Result};
use smallvec::SmallVec;

pub const DAY: Day = Day {
    day: 8,
    name: "Memory Maneuver",
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
struct NodeId(u16);

#[derive(Debug, Clone, Default)]
struct Node {
    children: SmallVec<[NodeId; 8]>,
    metadata: SmallVec<[u8; 16]>,
}

#[derive(Debug, Clone, Default)]
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new_node(&mut self) -> NodeId {
        let id = NodeId(self.nodes.len() as u16);
        self.nodes.push(Node::default());
        id
    }

    fn add_child(&mut self, parent: NodeId, child: NodeId) {
        self.nodes[parent.0 as usize].children.push(child);
    }

    fn add_metadata(&mut self, node: NodeId, metadata: u8) {
        self.nodes[node.0 as usize].metadata.push(metadata);
    }

    fn value_of(&self, cache: &mut [Option<u16>], node_id: NodeId) -> u16 {
        if let Some(val) = cache[node_id.0 as usize] {
            return val;
        }

        let node = &self.nodes[node_id.0 as usize];
        let value = if node.children.is_empty() {
            node.metadata.iter().map(|&m| m as u16).sum::<u16>()
        } else {
            node.metadata
                .iter()
                .map(|&m| {
                    if m == 0 {
                        0
                    } else if let Some(&child_id) = node.children.get((m - 1) as usize) {
                        self.value_of(cache, child_id)
                    } else {
                        0
                    }
                })
                .sum::<u16>()
        };

        cache[node_id.0 as usize] = Some(value);

        value
    }
}

fn parse_node(input: &mut impl Iterator<Item = u8>, tree: &mut Tree) -> Option<NodeId> {
    let num_children = input.next()?;
    let num_metadata = input.next()?;
    let node_id = tree.new_node();

    for _ in 0..num_children {
        let child_id = parse_node(input, tree)?;
        tree.add_child(node_id, child_id);
    }

    for _ in 0..num_metadata {
        tree.add_metadata(node_id, input.next()?);
    }

    Some(node_id)
}

fn parse(input: &str) -> Result<Tree> {
    let mut tree = Tree::default();

    let mut input = input
        .split_ascii_whitespace()
        .map(str::parse::<u8>)
        .filter_map(Result::ok);

    parse_node(&mut input, &mut tree).ok_or_else(|| eyre!("Error parsing input"))?;

    Ok(tree)
}

fn part1(tree: &Tree) -> u32 {
    tree.nodes
        .iter()
        .map(|n| n.metadata.iter().map(|&m| m as u32).sum::<u32>())
        .sum()
}

fn part2(tree: &Tree) -> u16 {
    let mut cache = vec![None; tree.nodes.len()];
    tree.value_of(&mut cache, NodeId(0))
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
        let expected = 138;
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
        let expected = 66;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }
}
