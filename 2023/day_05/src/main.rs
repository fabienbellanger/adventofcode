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

impl Category {
    fn sorted_vec() -> [Self; 7] {
        [
            Self::Soil,
            Self::Fertilizer,
            Self::Water,
            Self::Light,
            Self::Temperature,
            Self::Humidity,
            Self::Location,
        ]
    }
}

#[derive(Debug, Default, Clone)]
struct Almanac {
    seeds: Vec<Seed>,
    categories: HashMap<Category, Vec<Conversion>>,
}

#[derive(Debug, Default, Clone)]
struct Conversion {
    source_range: Range<usize>,
    destination_start: usize,
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Almanac) -> usize {
    let seeds = data.seeds;
    let categories = data.categories;
    let mut min = usize::MAX;
    let sorted_categories = Category::sorted_vec();

    for seed in seeds {
        let mut number = seed;

        for category in &sorted_categories {
            let conversions = categories.get(category).unwrap();
            let mut delta = 0isize;

            for conversion in conversions.iter() {
                if conversion.source_range.contains(&number) {
                    delta = conversion.destination_start as isize - conversion.source_range.start as isize;
                    break;
                }
            }

            number = (number as isize + delta) as usize;
        }

        if number < min {
            min = number;
        }
    }

    min
}

fn part2(data: Almanac) -> usize {
    let seeds = data.seeds;
    let categories = data.categories;
    let mut min = usize::MAX;
    let sorted_categories = Category::sorted_vec();

    let seeds_ranges = seeds
        .chunks(2)
        .map(|list| list[0]..(list[0] + list[1]))
        .collect::<Vec<_>>();

    for seed_range in seeds_ranges.into_iter() {
        for seed in seed_range {
            let mut number = seed;

            for category in &sorted_categories {
                let conversions = categories.get(category).unwrap();
                let mut delta = 0isize;

                for conversion in conversions {
                    if conversion.source_range.contains(&number) {
                        delta = conversion.destination_start as isize - conversion.source_range.start as isize;
                        break;
                    }
                }

                number = (number as isize + delta) as usize;
            }

            if number < min {
                min = number;
            }
        }
    }

    min
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
        .map(|part| {
            let mut lines = part.trim().lines().collect::<VecDeque<_>>();
            let header = lines.pop_front().unwrap();

            // Category
            let category = if header.contains("to-soil") {
                Category::Soil
            } else if header.contains("to-fertilizer") {
                Category::Fertilizer
            } else if header.contains("to-water") {
                Category::Water
            } else if header.contains("to-light") {
                Category::Light
            } else if header.contains("to-temperature") {
                Category::Temperature
            } else if header.contains("to-humidity") {
                Category::Humidity
            } else {
                Category::Location
            };

            // Ranges
            let mut conversions = vec![];
            for lines in lines {
                let values: Vec<_> = lines
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();

                let range = values[1]..(values[1] + values[2]);
                conversions.push(Conversion {
                    source_range: range,
                    destination_start: values[0],
                })
            }

            (category, conversions)
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
        assert_eq!(806_029_445, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2(parse_input(TEST)));
        assert_eq!(59_370_572, part2(parse_input(INPUT)));
    }
}
