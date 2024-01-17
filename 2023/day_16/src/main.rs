use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::{fmt, fs};
use utils::point::Point;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, PartialEq)]
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
    fn in_bounds(&self, point: Point) -> bool {
        todo!()
    }

    fn next_moves(&self, current: Point, direction: Direction) -> Vec<(Point, Direction)> {
        let mut moves = Vec::new();

        let current_element = self.elements.get(&current);
        match current_element {
            Some(element) => todo!(),
            None => todo!(),
        }

        moves
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Grid) -> usize {
    println!("{}", &data);

    0
}

fn part2(data: Grid) -> usize {
    todo!()
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
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
