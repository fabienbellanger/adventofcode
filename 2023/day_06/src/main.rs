#![allow(unused_variables)]
use std::fs;

const INPUT: &str = "input.txt";

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Vec<String>) -> usize {
    todo!()
}

fn part2(data: Vec<String>) -> usize {
    todo!()
}

fn parse_input(file: &str) -> Vec<String> {
    let lines = fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<_>>();
    dbg!(lines);

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(288, part1(parse_input(TEST)));
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
