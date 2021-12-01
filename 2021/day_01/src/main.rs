use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(lines: Vec<isize>) -> usize {
    lines.windows(2).filter(|s| s[1] > s[0]).count()
}

fn part2(lines: Vec<isize>) -> usize {
    lines
        .windows(3)
        .map(|s| s.iter().sum())
        .collect::<Vec<isize>>()
        .windows(2)
        .filter(|s| s[1] > s[0])
        .count()
}

#[test]
fn test_part1() {
    assert_eq!(
        7,
        part1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
    );
    assert_eq!(1139, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(
        5,
        part2(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
    );
    assert_eq!(1103, part2(get_data()));
}

fn get_data() -> Vec<isize> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
