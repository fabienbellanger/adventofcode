use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data("day_16/input.txt")));
    // println!("Part 2 result: {}", part2(get_data("day_16/input.txt")));
}

fn part1(data: Vec<String>) -> usize {
    dbg!(&data);
    0
}

// fn part2(data: Vec<String>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(1651, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect()
}
