use std::{collections::HashSet, fmt::Debug, fs};

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
enum Instruction {
    Horizontal(isize),
    Vertical(isize),
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn solve(data: (HashSet<Point>, Vec<Instruction>), part_1: bool) -> usize {
    let mut dots: HashSet<Point> = data.0;

    for (index, instruction) in data.1.into_iter().enumerate() {
        let mut new_dots = HashSet::new();

        match instruction {
            Instruction::Horizontal(y) => {
                for point in &dots {
                    let mut new_point = point.clone();
                    if point.y > y {
                        new_point = Point {
                            x: point.x,
                            y: y - (point.y - y),
                        };
                        new_dots.insert(new_point);
                    } else if point.y < y {
                        new_dots.insert(new_point);
                    }
                }
            }
            Instruction::Vertical(x) => {
                for point in &dots {
                    let mut new_point = point.clone();
                    if point.x > x {
                        new_point = Point {
                            x: x - (point.x - x),
                            y: point.y,
                        };
                        new_dots.insert(new_point);
                    } else if point.x < x {
                        new_dots.insert(new_point);
                    }
                }
            }
        }

        dots = new_dots;

        if part_1 && index == 0 {
            return dots.len();
        }
    }

    let max_x = *dots.iter().map(|Point { x, y: _ }| x).max().unwrap();
    let max_y = *dots.iter().map(|Point { x: _, y }| y).max().unwrap();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    dots.len()
}

fn part1(data: (HashSet<Point>, Vec<Instruction>)) -> usize {
    solve(data, true)
}

fn part2(data: (HashSet<Point>, Vec<Instruction>)) -> usize {
    solve(data, false)
}

#[test]
fn test_part1() {
    assert_eq!(17, part1(get_data("test.txt")));
    assert_eq!(847, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(16, part2(get_data("test.txt")));
    assert_eq!(104, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (HashSet<Point>, Vec<Instruction>) {
    let data = fs::read_to_string(file).unwrap_or_else(|_| panic!("Cannot read the file {}", file));
    let (dots, instructions) = data.split_once("\n\n").unwrap();

    let dots = dots
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            Point { x, y }
        })
        .collect::<HashSet<Point>>();

    let instructions = instructions
        .trim()
        .lines()
        .map(|line| {
            let (part1, value) = line.split_once('=').unwrap();
            let value = value.parse::<isize>().unwrap();

            match part1.contains('x') {
                true => Instruction::Vertical(value),
                false => Instruction::Horizontal(value),
            }
        })
        .collect::<Vec<Instruction>>();

    (dots, instructions)
}
