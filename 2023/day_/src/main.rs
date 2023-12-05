#![allow(unused_variables)]
use utils::data::file_to_vec_string;

const INPUT: &str = "input.txt";

fn main() {
    println!("Part 1 result: {}", part1(file_to_vec_string(INPUT)));
    println!("Part 2 result: {}", part2(file_to_vec_string(INPUT)));
}

fn part1(data: Vec<String>) -> u32 {
    todo!()
}

fn part2(data: Vec<String>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(file_to_vec_string(TEST)));
        // assert_eq!(0, part1(file_to_vec_string(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(file_to_vec_string(TEST)));
        // assert_eq!(0, part2(file_to_vec_string(INPUT)));
    }
}
