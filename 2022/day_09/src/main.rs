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

fn clamp(value: isize, min: isize, max: isize) -> isize {
    if value > max {
        return max;
    }

    if value < min {
        return min;
    }

    value
}

fn process(moves: Vec<Move>, n: usize) -> usize {
    let mut rope = vec![Point { x: 0, y: 0 }; n];
    let mut visited: HashSet<Point> = HashSet::new();

    for step in moves {
        for _ in 0..step.value {
            match step.direction {
                Direction::Right => rope[0].x += 1,
                Direction::Upper => rope[0].y -= 1,
                Direction::Left => rope[0].x -= 1,
                Direction::Down => rope[0].y += 1,
            }

            for i in 1..n {
                let p = &mut rope[i - 1..=i];

                let head = p[0];
                let mut tail = &mut p[1];

                if tail.near(&head) {
                    continue;
                }

                let diff_x = clamp(head.x - tail.x, -1, 1);
                let diff_y = clamp(head.y - tail.y, -1, 1);

                tail.x += diff_x;
                tail.y += diff_y;
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(moves: Vec<Move>) -> usize {
    process(moves, 2)
}

fn part2(moves: Vec<Move>) -> usize {
    process(moves, 10)
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
    assert_eq!(2445, part2(get_data("input.txt")));
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
