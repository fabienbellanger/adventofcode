use std::fmt::Formatter;
use std::{fmt, fs};

const INPUT: &str = "input.txt";

#[derive(Debug, Copy, Clone, PartialEq)]
enum Element {
    Miror1,
    Miror2,
    HSplitter,
    VSplitter,
    Empty,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '/' => Self::Miror1,
            '\\' => Self::Miror2,
            '|' => Self::VSplitter,
            '-' => Self::HSplitter,
            '.' => Self::Empty,
            v => panic!("invalid input: {v}"),
        }
    }
}

impl From<Element> for char {
    fn from(value: Element) -> Self {
        match value {
            Element::Miror1 => '/',
            Element::Miror2 => '\\',
            Element::VSplitter => '|',
            Element::HSplitter => '-',
            Element::Empty => '.',
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    elements: Vec<Vec<Element>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut lines = String::new();
        for y in 0..self.size {
            for x in 0..self.size {
                lines.push(self.elements[y][x].into());
            }
            lines.push('\n');
        }
        write!(f, "{lines}")
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
    let mut elements = Vec::new();

    let data = fs::read_to_string(file).unwrap_or_else(|_| panic!("Cannot read the file {file}"));

    for (y, line) in data.trim().lines().enumerate() {
        if y == 0 {
            size = line.len();
        }

        elements.push(Vec::new());

        for c in line.chars() {
            elements[y].push(Element::from(c));
        }
    }

    Grid { elements, size }
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
