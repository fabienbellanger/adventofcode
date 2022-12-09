use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Vec<usize>>) -> usize {
    data.iter()
        .map(|elf| elf.iter().sum::<usize>())
        .max()
        .unwrap_or_default()
}

fn part2(data: Vec<Vec<usize>>) -> usize {
    let mut elves: Vec<usize> = data.iter().map(|elf| elf.iter().sum::<usize>()).collect();
    elves.sort_unstable();
    elves.iter().rev().take(3).sum()
}

#[test]
fn test_part1() {
    assert_eq!(24_000, part1(get_data("test.txt")));
    assert_eq!(72_602, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(45_000, part2(get_data("test.txt")));
    assert_eq!(207_410, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Vec<usize>> {
    let parts = fs::read_to_string(file).expect("Cannot read the file input.txt");

    parts
        .trim()
        .split("\n\n")
        .map(|part| {
            part.trim()
                .split('\n')
                .map(|line| line.parse::<usize>().unwrap_or_default())
                .collect::<Vec<usize>>()
        })
        .collect()
}
