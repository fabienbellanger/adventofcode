use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Formatter;
use std::{fmt, fs};
use utils::point::Point;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Element {
    Slash,
    Backslash,
    HSplitter,
    VSplitter,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            '|' => Self::VSplitter,
            '-' => Self::HSplitter,
            v => panic!("invalid input: {v}"),
        }
    }
}

impl From<Element> for char {
    fn from(value: Element) -> Self {
        match value {
            Element::Slash => '/',
            Element::Backslash => '\\',
            Element::VSplitter => '|',
            Element::HSplitter => '-',
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    elements: HashMap<Point, Element>,
    energized: HashSet<Point>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut lines = String::new();
        for y in 0..self.size {
            for x in 0..self.size {
                let p = Point::new(x as isize, y as isize);
                if let Some(e) = self.elements.get(&p) {
                    lines.push((*e).into());
                } else if self.energized.get(&p).is_some() {
                    lines.push('#');
                } else {
                    lines.push('.');
                }
            }
            lines.push('\n');
        }
        write!(f, "{lines}")
    }
}

impl Grid {
    fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.size as isize && point.y >= 0 && point.y < self.size as isize
    }

    fn move_point(&self, point: &Point, direction: &Direction) -> Option<Point> {
        let next = match direction {
            Direction::North => Point::new(point.x, point.y - 1),
            Direction::South => Point::new(point.x, point.y + 1),
            Direction::East => Point::new(point.x + 1, point.y),
            Direction::West => Point::new(point.x - 1, point.y),
        };

        if self.in_bounds(&next) {
            return Some(next);
        }
        None
    }

    fn add_move(&self, moves: &mut Vec<(Point, Direction)>, current: &Point, direction: &Direction) {
        if let Some(point) = self.move_point(current, direction) {
            moves.push((point, direction.clone()));
        }
    }

    fn next_moves(&self, current: &Point, direction: &Direction) -> Vec<(Point, Direction)> {
        let mut moves = Vec::new();

        let current_element = self.elements.get(current);
        match current_element {
            Some(element) => match element {
                Element::Backslash => match direction {
                    Direction::North => self.add_move(&mut moves, current, &Direction::West),
                    Direction::South => self.add_move(&mut moves, current, &Direction::East),
                    Direction::East => self.add_move(&mut moves, current, &Direction::South),
                    Direction::West => self.add_move(&mut moves, current, &Direction::North),
                },
                Element::Slash => match direction {
                    Direction::North => self.add_move(&mut moves, current, &Direction::East),
                    Direction::South => self.add_move(&mut moves, current, &Direction::West),
                    Direction::East => self.add_move(&mut moves, current, &Direction::North),
                    Direction::West => self.add_move(&mut moves, current, &Direction::South),
                },
                Element::HSplitter => match direction {
                    Direction::North | Direction::South => {
                        self.add_move(&mut moves, current, &Direction::East);
                        self.add_move(&mut moves, current, &Direction::West);
                    }
                    _ => self.add_move(&mut moves, current, direction),
                },
                Element::VSplitter => match direction {
                    Direction::East | Direction::West => {
                        self.add_move(&mut moves, current, &Direction::North);
                        self.add_move(&mut moves, current, &Direction::South);
                    }
                    _ => self.add_move(&mut moves, current, direction),
                },
            },
            None => self.add_move(&mut moves, current, direction),
        }

        moves
    }

    fn process(&self, start: (Point, Direction)) -> usize {
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        let mut energized = self.energized.clone();

        visited.insert(start.clone());
        energized.insert(start.0.clone());
        let next = self.next_moves(&start.0, &start.1);
        for p in next {
            to_visit.push_back(p);
        }

        while let Some(current) = to_visit.pop_front() {
            if !visited.contains(&current) {
                visited.insert(current.clone());
                energized.insert(current.0.clone());

                let next = self.next_moves(&current.0, &current.1);
                for p in next {
                    to_visit.push_back(p);
                }
            }
        }

        energized.len()
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Grid) -> usize {
    data.process((Point::new(0, 0), Direction::East))
}

// 7746 => too high
fn part2(data: Grid) -> usize {
    let mut max = 0;

    for y in 0..data.size {
        for x in 0..data.size {
            // Only bounds
            if y == 0 || y == data.size - 1 || x == 0 || x == data.size - 1 {
                let point = Point::new(x as isize, y as isize);

                if !data.elements.contains_key(&point) {
                    let north_value = data.process((point.clone(), Direction::North));
                    let south_value = data.process((point.clone(), Direction::South));
                    let east_value = data.process((point.clone(), Direction::East));
                    let west_value = data.process((point.clone(), Direction::West));

                    max = *[north_value, south_value, east_value, west_value, max]
                        .iter()
                        .max()
                        .unwrap_or(&max);
                }
            }
        }
    }

    max
}

fn parse_input(file: &str) -> Grid {
    let mut size = 0;
    let mut elements = HashMap::new();

    let data = fs::read_to_string(file).unwrap_or_else(|_| panic!("Cannot read the file {file}"));

    for (y, line) in data.trim().lines().enumerate() {
        if y == 0 {
            size = line.len();
        }

        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                elements.insert(Point::new(x as isize, y as isize), Element::from(c));
            }
        }
    }

    Grid {
        elements,
        size,
        energized: HashSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(46, part1(parse_input(TEST)));
        assert_eq!(7_517, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(51, part2(parse_input(TEST)));
        assert_eq!(7_741, part2(parse_input(INPUT)));
    }
}
