use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(lines: Vec<usize>) -> isize {
    0
}

fn part2(lines: Vec<usize>) -> isize {
    0
}

#[test]
fn test_part1() {
    // assert_eq!(198, part1(movements));
    assert_eq!(198, part1(get_data()));
}

#[test]
fn test_part2() {
    // assert_eq!(900, part2(movements));
    assert_eq!(1880593125, part2(get_data()));
}

fn get_data() -> Vec<usize> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
