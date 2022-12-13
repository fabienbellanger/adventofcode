use std::{collections::HashSet, fmt::Display, fs};

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn near(&self, p: &Self) -> bool {
        self.x.abs_diff(p.x) <= 1 && self.y.abs_diff(p.y) <= 1
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
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

            if !tail.near(&head) {
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
    let mut rope = [Point { x: 0, y: 0 }; 10];
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point { x: 0, y: 0 });

    for step in moves {
        for _ in 0..step.value {}
    }

    visited.len()
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
