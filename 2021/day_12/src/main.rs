use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(paths: HashMap<String, Vec<String>>) -> usize {
    dbg!(&paths);
    0
}

fn part2() -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(10, part1(get_data("test.txt")));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> HashMap<String, Vec<String>> {
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .for_each(|line| {
            let (from, to) = line.split_once('-').unwrap();

            paths.entry(from.to_string()).or_default().push(to.to_string());

            paths.entry(to.to_string()).or_default().push(from.to_string());
        });

    paths
}
