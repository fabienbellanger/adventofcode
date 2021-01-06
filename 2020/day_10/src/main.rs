use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(&get_data()).unwrap());
    println!("Part 2 result: {}", part2(&get_data()).unwrap());
}

fn part1(data: &[usize]) -> Option<usize> {
    dbg!(data);
    None
}

fn part2(data: &[usize]) -> Option<usize> {
    None
}

fn get_data() -> Vec<usize> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn _get_data_test() -> Vec<usize> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(Some(22), part1(&_get_data_test()));
    // assert_eq!(Some(), part1(&get_data()));
}

#[test]
fn test_part2() {
    // assert_eq!(Some(), part2(&_get_data_test()));
    // assert_eq!(Some(), part2(&get_data()));
}
