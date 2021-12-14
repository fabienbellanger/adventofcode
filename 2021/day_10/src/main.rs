use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Vec<char>>) -> usize {
    let mut points: HashMap<char, usize> = HashMap::with_capacity(4);
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1_197);
    points.insert('>', 25_137);

    let mut errors: HashMap<char, usize> = HashMap::with_capacity(data.len());

    for line in data {
        let mut stack: Vec<char> = Vec::new();

        for c in line {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if let Some(last) = stack.pop() {
                        match (last, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => (),
                            _ => {
                                let err = errors.entry(c).or_insert(0);
                                *err += 1;
                                break;
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    errors.iter().map(|(c, n)| points.get(c).unwrap_or(&0) * *n).sum()
}

fn part2(data: Vec<Vec<char>>) -> usize {
    let mut missings: Vec<Vec<char>> = Vec::new();

    'line: for line in data {
        let mut stack: Vec<char> = Vec::new();

        for c in line {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if let Some(last) = stack.pop() {
                        match (last, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => (),
                            _ => {
                                continue 'line;
                            }
                        }
                    }
                }
                _ => panic!("invalid character"),
            }
        }

        let missing = stack
            .into_iter()
            .rev()
            .map(|c| match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => panic!("invalid character"),
            })
            .collect();
        missings.push(missing);
    }

    let mut scores: Vec<usize> = missings
        .into_iter()
        .map(|list| {
            list.into_iter().fold(0, |acc, c| match c {
                ')' => 5 * acc + 1,
                ']' => 5 * acc + 2,
                '}' => 5 * acc + 3,
                '>' => 5 * acc + 4,
                _ => panic!("invalid character"),
            })
        })
        .collect();
    scores.sort_unstable();

    *scores.get(scores.len() / 2).unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(26397, part1(get_data("test.txt")));
    assert_eq!(389589, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(288957, part2(get_data("test.txt")));
    assert_eq!(1190420163, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|n| n.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
