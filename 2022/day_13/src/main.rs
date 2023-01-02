use serde::Deserialize;
use std::{cmp::Ordering, fmt, fs};

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(usize),
    List(Vec<Node>),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::List(l) => f.debug_list().entries(l).finish(),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (left, right) => {
                let left = match left {
                    Node::Number(n) => vec![Self::Number(*n)],
                    Node::List(l) => l.to_vec(),
                };
                let right = match right {
                    Node::Number(n) => vec![Self::Number(*n)],
                    Node::List(l) => l.to_vec(),
                };

                Some(
                    left.iter()
                        .zip(right.iter())
                        .map(|(l, r)| l.cmp(r))
                        .find(|&ord| ord != Ordering::Equal)
                        .unwrap_or_else(|| left.len().cmp(&right.len())),
                )
            }
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<(Node, Node)>) -> usize {
    data.into_iter()
        .enumerate()
        .filter_map(|(i, (left, right))| match left.cmp(&right) {
            Ordering::Less => Some(i + 1),
            _ => None,
        })
        .sum()
}

// fn part2(data: Vec<(Node, Node)>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(13, part1(get_data("test.txt")));
    assert_eq!(6101, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Vec<(Node, Node)> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .split("\n\n")
        .map(|line| {
            let (left, right) = line.trim().split_once('\n').unwrap();

            let left = serde_json::from_str::<Node>(left.trim()).unwrap();
            let right = serde_json::from_str::<Node>(right.trim()).unwrap();

            (left, right)
        })
        .collect()
}
