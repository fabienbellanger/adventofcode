use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: (String, HashMap<String, String>)) -> usize {
    0
}

fn part2(data: (String, HashMap<String, String>)) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(1588, part1(get_data("test.txt")));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (String, HashMap<String, String>) {
    let mut data: Vec<String> = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|n| n.to_string())
        .collect();

    let polymer = data.first().unwrap().clone();

    let pairs: HashMap<String, String> = data
        .into_iter()
        .skip(2)
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            (from.to_string(), to.to_string())
        })
        .collect();

    (polymer, pairs)
}
