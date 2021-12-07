use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(positions: Vec<usize>) -> usize {
    0
}

fn part2(positions: Vec<usize>) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(37, part1(get_data("test.txt")));
    // assert_eq!(365862, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(26984457539, part2(get_data("test.txt")));
    // assert_eq!(1653250886439, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<usize> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
