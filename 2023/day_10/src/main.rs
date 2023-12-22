#![allow(unused_variables)]
use std::collections::{HashMap, HashSet, VecDeque};
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
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
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
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
                Direction::Up => vec![Self::Start, Self::NorthSouth, Self::SouthWest, Self::SouthEast],
                Direction::Down => vec![Self::Start, Self::NorthSouth, Self::NorthWest, Self::NorthEast],
            },
            Self::EastWest => match direction {
                Direction::Up | Direction::Down => vec![],
                Direction::Left => vec![Self::Start, Self::EastWest, Self::NorthEast, Self::SouthEast],
                Direction::Right => vec![Self::Start, Self::EastWest, Self::NorthWest, Self::SouthWest],
            },
            Self::NorthEast => match direction {
                Direction::Left | Direction::Down => vec![],
                Direction::Up => vec![Self::Start, Self::NorthSouth, Self::SouthWest, Self::SouthEast],
                Direction::Right => vec![Self::Start, Self::EastWest, Self::NorthWest, Self::SouthWest],
            },
            Self::NorthWest => match direction {
                Direction::Right | Direction::Down => vec![],
                Direction::Up => vec![Self::Start, Self::NorthSouth, Self::SouthWest, Self::SouthEast],
                Direction::Left => vec![Self::Start, Self::EastWest, Self::NorthEast, Self::SouthEast],
            },
            Self::SouthEast => match direction {
                Direction::Left | Direction::Up => vec![],
                Direction::Down => vec![Self::Start, Self::NorthSouth, Self::NorthWest, Self::NorthEast],
                Direction::Right => vec![Self::Start, Self::EastWest, Self::NorthWest, Self::SouthWest],
            },
            Self::SouthWest => match direction {
                Direction::Right | Direction::Up => vec![],
                Direction::Down => vec![Self::Start, Self::NorthSouth, Self::NorthWest, Self::NorthEast],
                Direction::Left => vec![Self::Start, Self::EastWest, Self::NorthEast, Self::SouthEast],
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
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct TilePoint {
    tile: Tile,
    point: Point,
}

impl fmt::Display for TilePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.tile, self.point.x, self.point.y)
    }
}

#[derive(Debug, Default, Clone)]
struct Grid {
    tiles: HashMap<Point, TilePoint>,
    main_loop: HashSet<TilePoint>,
    start: TilePoint,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Grid {
    fn find_tiles_available(&self, current: &TilePoint) -> HashSet<TilePoint> {
        let mut list = HashSet::new();

        for direction in DIRECTIONS {
            let (dx, dy) = direction.clone().into();
            let x = current.point.x + dx;
            let y = current.point.y + dy;

            if x < 0 || y < 0 {
                continue;
            }

            if let Some(tile_point) = self.tiles.get(&Point::new(x, y)) {
                let tiles_available = current.tile.tiles_available(direction);

                for tile in tiles_available {
                    if tile == tile_point.tile {
                        list.insert(tile_point.clone());
                    }
                }
            }
        }

        list
    }

    fn find_loop(&mut self, start: TilePoint) {
        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();

        let mut current = start.clone();
        visited.insert(current.clone());

        loop {
            let available_tiles = self.find_tiles_available(&current);

            for tile in available_tiles {
                if !visited.contains(&tile) {
                    to_visit.push_front(tile);
                }
            }

            if let Some(c) = to_visit.pop_front() {
                current = c;
                visited.insert(current.clone());
            } else {
                break;
            }
        }

        self.main_loop = visited;
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(mut grid: Grid) -> usize {
    grid.find_loop(grid.start.clone());
    grid.main_loop.len() / 2
}

fn part2(data: Grid) -> usize {
    todo!()
}

fn parse_input(file: &str) -> Grid {
    let data = fs::read_to_string(file).expect(&format!("Cannot read the file {file}"));

    let mut tiles = HashMap::new();
    let mut main_loop = HashSet::new();
    let mut start = TilePoint {
        tile: Tile::Start,
        point: Point::new(0, 0),
    };
    for (y, line) in data.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let tile = Tile::try_from(c).unwrap();
            let point = Point::new(x as isize, y as isize);

            if tile != Tile::Ground {
                let tile_point = TilePoint {
                    point: point.clone(),
                    tile: tile.clone(),
                };
                if tile == Tile::Start {
                    start = tile_point.clone();
                    main_loop.insert(start.clone());
                }
                tiles.insert(point, tile_point);
            }
        }
    }

    Grid {
        tiles,
        main_loop,
        start,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1: &str = "test_1.txt";
    const TEST_2: &str = "test_2.txt";

    #[test]
    fn test_part1() {
        assert_eq!(4, part1(parse_input(TEST_1)));
        assert_eq!(8, part1(parse_input(TEST_2)));
        assert_eq!(6_815, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST_1)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
