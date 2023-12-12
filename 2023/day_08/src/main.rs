#![allow(unused_variables)]
use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT: &str = "input.txt";

type Node = String;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<Node, (Node, Node)>,
    current_nodes: Vec<Node>,
    steps: Vec<usize>,
    ends: Vec<bool>,
    end_nodes: HashSet<Node>,
}

impl Network {
    fn next(&mut self, direction: Direction) {
        for i in 0..self.current_nodes.len() {
            let (left, right) = self.nodes.get(self.current_nodes.get(i).unwrap()).unwrap().clone();

            self.current_nodes[i] = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };
            self.steps[i] += 1;

            if self.end_nodes.contains(&self.current_nodes[i]) {
                self.ends[i] = true;
            } else {
                self.ends[i] = false;
            }
        }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT), "AAA", "ZZZ"));
    println!("Part 2 result: {}", part2(parse_input(INPUT), 'A', 'Z'));
}

fn part1(data: (Vec<Direction>, Network), start: &str, end: &str) -> usize {
    let mut network = data.1;
    network.current_nodes.push(start.to_string());
    network.steps.push(0);
    network.ends.push(false);
    network.end_nodes = HashSet::from([end.to_string()]);

    for d in data.0.into_iter().cycle() {
        if network.ends.iter().all(|b| *b == true) {
            break;
        }
        network.next(d);
    }

    network.steps[0]
}

fn part2(data: (Vec<Direction>, Network), start: char, end: char) -> usize {
    let mut network = data.1;

    network.current_nodes = network
        .nodes
        .iter()
        .filter(|&(node, _)| node.ends_with(start))
        .map(|(node, _)| node)
        .cloned()
        .collect::<Vec<_>>();

    network.end_nodes = network
        .nodes
        .iter()
        .filter(|&(node, _)| node.ends_with(end))
        .map(|(node, _)| node)
        .cloned()
        .collect::<HashSet<Node>>();

    network.steps = (0..network.current_nodes.len()).map(|_| 0).collect();
    network.ends = (0..network.current_nodes.len()).map(|_| false).collect();

    for d in data.0.into_iter().cycle() {
        let finished = network.ends.iter().all(|b| *b == true);
        if finished {
            break;
        }
        network.next(d);
    }

    network.steps.into_iter().max().unwrap_or_default()
}

fn parse_input(file: &str) -> (Vec<Direction>, Network) {
    let data = fs::read_to_string(file).expect(&format!("Cannot read the file {file}"));
    let (directions, data) = data.split_once("\n\n").unwrap();

    // Directions
    let directions = directions
        .chars()
        .map(|c| Direction::try_from(c).unwrap())
        .collect::<Vec<_>>();

    // Network
    let network = data
        .trim()
        .lines()
        .map(|line| {
            // AAA = (BBB, CCC)
            let line = line.strip_suffix(')').unwrap();
            let (key, parts) = line.trim().split_once(" = (").unwrap();
            let (left, right) = parts.trim().split_once(", ").unwrap();

            (key.to_string(), (left.to_string(), right.to_string()))
        })
        .collect();

    (
        directions,
        Network {
            nodes: network,
            current_nodes: vec![],
            steps: vec![],
            ends: vec![],
            end_nodes: HashSet::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1: &str = "test_1.txt";
    const TEST_2: &str = "test_2.txt";
    const TEST_3: &str = "test_3.txt";

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(parse_input(TEST_1), "AAA", "ZZZ"));
        assert_eq!(6, part1(parse_input(TEST_2), "AAA", "ZZZ"));
        assert_eq!(11_309, part1(parse_input(INPUT), "AAA", "ZZZ"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(parse_input(TEST_3), 'A', 'Z'));
        assert_eq!(0, part2(parse_input(INPUT), 'A', 'Z'));
    }
}
