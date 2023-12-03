#![allow(unused_variables)]
use std::collections::HashMap;
use std::fs;

const INPUT: &str = "day_02/input.txt";

#[derive(Debug, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

type Set = HashMap<Color, usize>;

#[derive(Debug)]
struct Game {
    number: usize,
    sets: Vec<Set>,
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Vec<Game>) -> u32 {
    dbg!(&data);

    0
}

fn part2(data: Vec<Game>) -> u32 {
    todo!()
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

fn parse_input(file: &str) -> Vec<Game> {
    let mut games = vec![];

    let input = fs::read_to_string(file)
        .expect("Cannot read the file");

    for (i, line) in input.trim().lines().enumerate() {
        dbg!(line);
    }

    return games
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "day_02/test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(parse_input(TEST)));
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
