use std::{fs, vec};

fn main() {
    // println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1() -> usize {
    0
}

fn part2() -> usize {
    0
}

#[test]
fn test_part1() {
    // assert_eq!(5, part1(get_data("test.txt")));
    // assert_eq!(5835, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(12, part2(get_data("test.txt")));
    // assert_eq!(17013, part2(get_data("input.txt")));
}

fn get_data(file: &str) {
    let data = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
}