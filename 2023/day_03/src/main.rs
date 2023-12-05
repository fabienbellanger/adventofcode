#![allow(unused_variables)]
use std::collections::{HashMap, HashSet};
use std::{fmt, fs};

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

type Symbols = HashMap<Point, char>;

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
    fn neighbors(&self) -> HashSet<Point> {
        let mut list = HashSet::new();

        for x in 0..self.value.len() {
            let p = Point::new(self.origin.x + x as isize, self.origin.y);
            let neighbors = p.neighbors();
            list.extend(neighbors.into_iter());
        }

        list
    }

    fn points(&self) -> Vec<Point> {
        let mut points = vec![self.origin.clone()];

        for x in 1..self.value.len() {
            points.push(Point::new(self.origin.x + x as isize, self.origin.y));
        }

        points
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: (Vec<PartNumber>, Symbols)) -> usize {
    let part_numbers = data.0;
    let symbols = data.1;

    part_numbers
        .iter()
        .filter_map(
            |part_number| match part_number.neighbors().iter().any(|point| symbols.get(point).is_some()) {
                false => None,
                true => Some(part_number.value.parse::<usize>().unwrap_or_default()),
            },
        )
        .sum()
}

fn part2(data: (Vec<PartNumber>, Symbols)) -> usize {
    let part_numbers = data.0;
    let gears: HashSet<Point> = data
        .1
        .iter()
        .filter_map(|(point, c)| if *c == '*' { Some(point) } else { None })
        .cloned()
        .collect();

    gears
        .into_iter()
        .filter_map(|gear| {
            let mut adjacents: Vec<usize> = vec![];

            for number in part_numbers.iter() {
                let neighbors = gear.neighbors();
                let points = number.points();
                let mut is_adjacent = false;

                for point in points {
                    if neighbors.contains(&point) {
                        is_adjacent = true;
                        break;
                    }
                }

                if is_adjacent {
                    adjacents.push(number.value.parse().unwrap_or_default());
                }
            }

            if adjacents.len() == 2 {
                Some(adjacents.into_iter().product::<usize>())
            } else {
                None
            }
        })
        .sum()
}

fn parse_input(file: &str) -> (Vec<PartNumber>, Symbols) {
    let lines = fs::read_to_string(file).expect("Cannot read the file input.txt");

    let mut part_numbers = vec![];
    let mut symbols = HashMap::new();

    for (y, line) in lines.trim().lines().enumerate() {
        let mut current_number = String::new();
        let mut current_origin = Point::new(0, y as isize);
        let line_size = line.len();

        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if current_number.is_empty() {
                    current_origin.x = x as isize;
                }
                current_number.push(c);

                // Last char of the line
                if x == line_size - 1 {
                    let part_number = PartNumber {
                        value: current_number.clone(),
                        origin: current_origin.clone(),
                    };
                    part_numbers.push(part_number);
                }
            } else {
                if c != '.' {
                    symbols.insert(Point::new(x as isize, y as isize), c);
                }

                if !current_number.is_empty() {
                    let part_number = PartNumber {
                        value: current_number.clone(),
                        origin: current_origin.clone(),
                    };
                    part_numbers.push(part_number);

                    current_number.clear();
                }
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
        assert_eq!(543867, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(467_835, part2(parse_input(TEST)));
        assert_eq!(79_613_331, part2(parse_input(INPUT)));
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
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
        ]);

        assert_eq!(part_number.neighbors(), expected);
    }
}
