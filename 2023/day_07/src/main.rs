#![allow(unused_variables)]
use std::fs;
use utils::data::file_to_vec_string;

const INPUT: &str = "input.txt";

fn main() {
    println!("Part 1 result: {}", part1(file_to_vec_string(INPUT)));
    println!("Part 2 result: {}", part2(file_to_vec_string(INPUT)));
}

fn part1(data: Vec<String>) -> usize {
    todo!()
}

fn part2(data: Vec<String>) -> usize {
    todo!()
}

// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
fn parse_input(file: &str) {
    let lines = fs::read_to_string(file)
        .expect(&format!("Cannot read the file {file}"))
        .lines()
        .map(|line| {
            let (cards, value) = line.trim().split_once(' ').unwrap();

            0
        })
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(6440, part1(file_to_vec_string(TEST)));
        // assert_eq!(0, part1(file_to_vec_string(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(file_to_vec_string(TEST)));
        // assert_eq!(0, part2(file_to_vec_string(INPUT)));
    }
}
