use std::{cmp::min, collections::HashSet, fmt::Debug, fs};

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
enum Instruction {
    horizontal(usize),
    vertical(usize),
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: (HashSet<Point>, Vec<Instruction>, Point)) -> usize {
    dbg!(&data);
    let mut points = data.0;
    let mut max_point = data.2;

    for instruction in data.1 {
        // 1. Calculer le delta entre N - y
        // 2. Changer les points entre delta..N pour les mettre dans 0..delta
        // 3. Supprimer la parte delta..N et la ligne du pli
        // 4. Calculter nouveau max_point
        match instruction {
            Instruction::horizontal(y) => {
                let range_y = if y < max_point.y - y {
                    0..y
                } else {
                    y + 1..max_point.y + 1
                };
                dbg!(range_y);
            }
            Instruction::vertical(x) => {
                let range_x = if x < max_point.x - x {
                    0..x
                } else {
                    x + 1..max_point.x + 1
                };
                dbg!(range_x);
            }
        }
    }
    0
}

fn part2() -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(17, part1(get_data("test.txt")));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (HashSet<Point>, Vec<Instruction>, Point) {
    let data = fs::read_to_string(file).unwrap_or_else(|_| panic!("Cannot read the file {}", file));
    let (dots, instructions) = data.split_once("\n\n").unwrap();

    let mut max_point = Point { x: 0, y: 0 };
    let dots = dots
        .trim()
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();

            if x > max_point.x {
                max_point.x = x;
            }
            if y > max_point.y {
                max_point.y = y;
            }

            Point { x, y }
        })
        .collect::<HashSet<Point>>();

    let instructions = instructions
        .trim()
        .lines()
        .map(|line| {
            let (part1, value) = line.split_once('=').unwrap();
            let value = value.parse::<usize>().unwrap();

            match part1.contains("x") {
                true => Instruction::vertical(value),
                false => Instruction::horizontal(value),
            }
        })
        .collect::<Vec<Instruction>>();

    (dots, instructions, max_point)
}
