use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt"), 10));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: (Vec<char>, HashMap<(char, char), char>), steps: usize) -> usize {
    dbg!((&data, steps));

    0
}

fn part2(data: (Vec<char>, HashMap<(char, char), char>)) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(1588, part1(get_data("test.txt"), 1));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let data: Vec<String> = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|n| n.to_string())
        .collect();

    let polymer = data.first().unwrap().chars().collect();

    let pairs = data
        .into_iter()
        .skip(2)
        .map(|line| {
            let (pattern, new) = line.split_once(" -> ").unwrap();
            let mut pattern = pattern.chars();
            let p1 = pattern.next().unwrap();
            let p2 = pattern.next().unwrap();

            let new = new.chars().next().unwrap();
            ((p1, p2), new)
        })
        .collect();

    (polymer, pairs)
}
