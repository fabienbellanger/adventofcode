#![allow(unused_variables)]
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::ops::Range;

const INPUT: &str = "input.txt";

type Seed = usize;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Category {
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Default, Clone)]
struct Almanac {
    seeds: Vec<Seed>,
    categories: HashMap<Category, Vec<Range<usize>>>,
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Almanac) -> u32 {
    dbg!(&data);
    todo!()
}

fn part2(data: Almanac) -> u32 {
    todo!()
}

fn parse_input(file: &str) -> Almanac {
    let data = fs::read_to_string(file).expect("Cannot read the file input.txt");
    let mut parts = data.trim().split("\n\n");

    let seeds = parts
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let categories = parts
        .into_iter()
        .map(|part| {
            let mut lines = part.trim().lines().collect::<VecDeque<_>>();
            let header = lines.pop_front().unwrap();

            // Category
            let category = if header.contains("soil") {
                Category::Soil
            } else if header.contains("fertilizer") {
                Category::Fertilizer
            } else if header.contains("water") {
                Category::Water
            } else if header.contains("light") {
                Category::Light
            } else if header.contains("temperature") {
                Category::Temperature
            } else if header.contains("humidity") {
                Category::Humidity
            } else {
                Category::Location
            };

            // Ranges

            (category, vec![])
        })
        .collect();

    Almanac { seeds, categories }
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(parse_input(TEST)));
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
