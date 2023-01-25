use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            c => Err(c),
        }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(jets: Vec<Direction>) -> usize {
    dbg!(&jets);
    0
}

// fn part2(data: Vec<String>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(3068, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Vec<Direction> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect()
}
