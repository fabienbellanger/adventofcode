use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Vec<char>>) -> usize {
    dbg!(&data);
    0
}

// fn part2(data: Vec<Vec<char>>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(157, part1(get_data("test.txt")));
    //assert_eq!(13446, part1(get_data("input.txt"))); // 12936
}

// #[test]
// #[ignore]
// fn test_part2() {
//     assert_eq!(12, part2(get_data("test.txt")));
//     assert_eq!(13509, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}
