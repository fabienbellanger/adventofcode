#![allow(unused_variables)]
use std::collections::HashSet;
use std::{fmt, fs};
use utils::data::file_to_vec_string;

const INPUT: &str = "input.txt";

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self) -> HashSet<Point> {
        let directions = [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];

        directions
            .iter()
            .map(|(d_x, d_y)| (*d_x + self.x, *d_y + self.y))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| Point::new(x, y))
            .collect()
    }
}

type Symbols = HashSet<Point>;

#[derive(Debug, Default, Clone)]
struct PartNumber {
    value: String,
    origin: Point,
}

impl fmt::Display for PartNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} from {})", self.value, self.origin)
    }
}

impl PartNumber {
    // TODO: Add tests
    fn neighbors(&self) -> HashSet<Point> {
        let mut list = HashSet::new();

        for x in 0..self.value.len() {
            let p = Point::new(self.origin.x + x as isize, self.origin.y);
            let neighbors = p.neighbors();
            list.extend(neighbors.into_iter());
        }

        list
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(file_to_vec_string(INPUT)));
}

fn part1(data: (Vec<PartNumber>, Symbols)) -> usize {
    let part_numbers = data.0;
    let symbols = data.1;

    part_numbers
        .iter()
        .inspect(|s| println!("{s}"))
        .filter_map(
            |part_number| match part_number.neighbors().iter().any(|point| symbols.contains(point)) {
                false => None,
                true => Some(part_number.value.parse::<usize>().unwrap_or_default()),
            },
        )
        .inspect(|s| println!("  => {s}"))
        .sum()
}

fn part2(data: Vec<String>) -> u32 {
    todo!()
}

fn parse_input(file: &str) -> (Vec<PartNumber>, Symbols) {
    let lines = fs::read_to_string(file).expect("Cannot read the file input.txt");

    let mut part_numbers = vec![];
    let mut symbols = HashSet::new();

    for (y, line) in lines.trim().lines().enumerate() {
        let mut current_number = String::new();
        let mut current_origin = Point::new(0, y as isize);

        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if current_number.is_empty() {
                    current_origin.x = x as isize;
                }
                current_number.push(c);
            } else if !current_number.is_empty() {
                let part_number = PartNumber {
                    value: current_number.clone(),
                    origin: current_origin.clone(),
                };
                part_numbers.push(part_number);

                current_number.clear();
            }

            if !c.is_ascii_digit() && c != '.' {
                symbols.insert(Point::new(x as isize, y as isize));
            }
        }
    }

    (part_numbers, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(4361, part1(parse_input(TEST)));
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(file_to_vec_string(TEST)));
        // assert_eq!(0, part2(file_to_vec_string(INPUT)));
    }

    #[test]
    fn test_point_neighbors() {
        let p = Point::new(0, 0);
        assert_eq!(p.neighbors().len(), 3);

        let p = Point::new(0, 1);
        assert_eq!(p.neighbors().len(), 5);

        let p = Point::new(1, 1);
        assert_eq!(p.neighbors().len(), 8);
    }

    #[test]
    fn test_part_number_neighbors() {
        let part_number = PartNumber {
            value: "467".to_string(),
            origin: Point::new(0, 0),
        };
        let expected = HashSet::from([
            Point::new(3, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1),
        ]);

        assert_eq!(part_number.neighbors(), expected);
    }
}
