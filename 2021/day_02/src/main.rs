use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Direction {
    Forward(isize),
    Up(isize),
    Down(isize),
}

#[derive(Debug)]
struct ParseDirectionError {}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            None => Err(Self::Err {}),
            Some(s) => match (s.0, s.1.parse().unwrap()) {
                ("forward", value) => Ok(Direction::Forward(value)),
                ("down", value) => Ok(Direction::Down(value)),
                ("up", value) => Ok(Direction::Up(value)),
                _ => Err(Self::Err {}),
            },
        }
    }
}

#[derive(Default)]
struct Position {
    depth: isize,
    horizontal: isize,
    aim: isize,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(lines: Vec<Direction>) -> isize {
    let mut position = Position::default();

    for line in lines {
        match line {
            Direction::Forward(value) => position.horizontal += value,
            Direction::Down(value) => position.depth += value,
            Direction::Up(value) => position.depth -= value,
        }
    }

    position.depth * position.horizontal
}

fn part2(lines: Vec<Direction>) -> isize {
    let mut position = Position::default();

    for line in lines {
        match line {
            Direction::Forward(value) => {
                position.horizontal += value;
                position.depth += position.aim * value;
            }
            Direction::Down(value) => position.aim += value,
            Direction::Up(value) => position.aim -= value,
        }
    }

    position.depth * position.horizontal
}

const _DIRECTIONS_TEST: [Direction; 6] = [
    Direction::Forward(5),
    Direction::Down(5),
    Direction::Forward(8),
    Direction::Up(3),
    Direction::Down(8),
    Direction::Forward(2),
];

#[test]
fn test_part1() {
    assert_eq!(150, part1(_DIRECTIONS_TEST.to_vec()));
    assert_eq!(1507611, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(900, part2(_DIRECTIONS_TEST.to_vec()));
    assert_eq!(1880593125, part2(get_data()));
}

fn get_data() -> Vec<Direction> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
