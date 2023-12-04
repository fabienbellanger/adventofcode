#![allow(unused_variables)]
use std::collections::HashSet;
use std::fs;
use utils::data::file_to_vec_string;

const INPUT: &str = "input.txt";

#[derive(Debug, Default, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

type Symbols = HashSet<Point>;

#[derive(Debug, Default, Clone)]
struct PartNumber {
    value: String,
    origin: Point,
}

impl PartNumber {
    // TODO
}

fn main() {
    println!("Part 1 result: {}", part1(file_to_vec_string(INPUT)));
    println!("Part 2 result: {}", part2(file_to_vec_string(INPUT)));
}

fn part1(data: Vec<String>) -> u32 {
    todo!()
}

fn part2(data: Vec<String>) -> u32 {
    todo!()
}

fn parse_input(file: &str) -> (Vec<PartNumber>, Symbols) {
    let lines = fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines();

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(4361, part1(file_to_vec_string(TEST)));
        // assert_eq!(0, part1(file_to_vec_string(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(file_to_vec_string(TEST)));
        // assert_eq!(0, part2(file_to_vec_string(INPUT)));
    }
}
