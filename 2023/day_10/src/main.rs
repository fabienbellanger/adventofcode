#![allow(unused_variables)]
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::{fmt, fs};

const INPUT: &str = "input.txt";
const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<Direction> for (isize, isize) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    #[default]
    Ground,
    Start,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NorthSouth => write!(f, "│"),
            Self::EastWest => write!(f, "─"),
            Self::NorthEast => write!(f, "┕"),
            Self::NorthWest => write!(f, "┘"),
            Self::SouthEast => write!(f, "┌"),
            Self::SouthWest => write!(f, "┐"),
            Self::Ground => write!(f, "."),
            Self::Start => write!(f, "S"),
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthEast),
            'F' => Ok(Self::SouthWest),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(()),
        }
    }
}

impl Tile {
    fn tiles_available(&self, direction: Direction) -> Vec<Self> {
        match self {
            Self::NorthSouth => match direction {
                Direction::Left | Direction::Right => vec![],
                Direction::Up => vec![Self::NorthSouth, Self::SouthWest, Self::SouthEast],
                Direction::Down => vec![Self::NorthSouth, Self::NorthWest, Self::NorthEast],
            },
            Self::EastWest => match direction {
                Direction::Up | Direction::Down => vec![],
                Direction::Left => vec![Self::EastWest, Self::NorthEast, Self::SouthEast],
                Direction::Right => vec![Self::EastWest, Self::NorthWest, Self::SouthWest],
            },
            Self::NorthEast => match direction {
                Direction::Left | Direction::Down => vec![],
                Direction::Up => vec![Self::NorthSouth, Self::SouthWest, Self::SouthEast],
                Direction::Right => vec![Self::EastWest, Self::NorthWest, Self::SouthWest],
            },
            Self::NorthWest => match direction {
                Direction::Right | Direction::Down => vec![],
                Direction::Up => vec![Self::NorthSouth, Self::SouthWest, Self::SouthEast],
                Direction::Left => vec![Self::EastWest, Self::NorthEast, Self::SouthEast],
            },
            Self::SouthEast => match direction {
                Direction::Left | Direction::Up => vec![],
                Direction::Down => vec![Self::NorthSouth, Self::NorthWest, Self::NorthEast],
                Direction::Right => vec![Self::EastWest, Self::NorthWest, Self::SouthWest],
            },
            Self::SouthWest => match direction {
                Direction::Right | Direction::Up => vec![],
                Direction::Down => vec![Self::NorthSouth, Self::NorthWest, Self::NorthEast],
                Direction::Left => vec![Self::EastWest, Self::NorthEast, Self::SouthEast],
            },
            Self::Start => match direction {
                Direction::Up => vec![Self::NorthSouth, Self::SouthWest, Self::SouthEast],
                Direction::Down => vec![Self::NorthSouth, Self::NorthWest, Self::NorthEast],
                Direction::Right => vec![Self::EastWest, Self::NorthWest, Self::SouthWest],
                Direction::Left => vec![Self::EastWest, Self::NorthEast, Self::SouthEast],
            },
            _ => vec![],
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Clone)]
struct Grid {
    tiles: HashMap<Point, Tile>,
    main_loop: HashSet<Point>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Grid {
    fn find_main_loop(&mut self) {
        //
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Grid) -> usize {
    dbg!(data);
    todo!()
}

fn part2(data: Grid) -> usize {
    todo!()
}

fn parse_input(file: &str) -> Grid {
    let data = fs::read_to_string(file).expect(&format!("Cannot read the file {file}"));

    let mut tiles = HashMap::new();
    let mut main_loop = HashSet::new();
    for (y, line) in data.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let tile = Tile::try_from(c).unwrap();
            let point = Point::new(x, y);

            if tile != Tile::Ground {
                if tile == Tile::Start {
                    main_loop.insert(point.clone());
                }
                tiles.insert(point, tile);
            }
        }
    }

    Grid { tiles, main_loop }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1: &str = "test_1.txt";
    const TEST_2: &str = "test_2.txt";

    #[test]
    fn test_part1() {
        assert_eq!(4, part1(parse_input(TEST_1)));
        // assert_eq!(8, part1(parse_input(TEST_2)));
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST_1)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
