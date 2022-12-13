use std::{collections::HashSet, fs};

#[derive(Debug)]
enum Direction {
    Right,
    Upper,
    Left,
    Down,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    value: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn is_tail_touching(head: &Point, tail: &Point) -> bool {
    tail.x.abs_diff(head.x) <= 1 && tail.y.abs_diff(head.y) <= 1
}

fn part1(moves: Vec<Move>) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(tail.clone());

    for step in moves {
        for _ in 0..step.value {
            match step.direction {
                Direction::Right => head.x += 1,
                Direction::Upper => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Down => head.y += 1,
            }

            if !is_tail_touching(&head, &tail) {
                match step.direction {
                    Direction::Right => {
                        tail = Point {
                            x: head.x - 1,
                            y: head.y,
                        }
                    }
                    Direction::Upper => {
                        tail = Point {
                            x: head.x,
                            y: head.y + 1,
                        }
                    }
                    Direction::Left => {
                        tail = Point {
                            x: head.x + 1,
                            y: head.y,
                        }
                    }
                    Direction::Down => {
                        tail = Point {
                            x: head.x,
                            y: head.y - 1,
                        }
                    }
                }

                visited.insert(tail.clone());
            }
        }
    }

    visited.len()
}

fn part2(moves: Vec<Move>) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(13, part1(get_data("test.txt")));
    assert_eq!(5902, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(1, part2(get_data("test.txt")));
    assert_eq!(36, part2(get_data("test2.txt")));
    // assert_eq!(0, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Move> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| {
            let (d, v) = line.trim().split_once(' ').unwrap();
            let value = v.parse().unwrap();
            match d {
                "R" => Move {
                    direction: Direction::Right,
                    value,
                },
                "U" => Move {
                    direction: Direction::Upper,
                    value,
                },
                "L" => Move {
                    direction: Direction::Left,
                    value,
                },
                "D" => Move {
                    direction: Direction::Down,
                    value,
                },
                _ => panic!(),
            }
        })
        .collect()
}
