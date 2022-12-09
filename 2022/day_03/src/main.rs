use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

// a => 97 => 1
// A => 65 => 27
fn convert_char(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 97 + 1,
        'A'..='Z' => c as usize - 65 + 27,
        _ => 0,
    }
}

fn part1(data: Vec<Vec<char>>) -> usize {
    data.iter()
        .map(|line| {
            let mid = line.len() / 2;
            let mut res = ' ';
            for c in &line[..mid] {
                if line[mid..].contains(c) {
                    res = *c;
                    break;
                }
            }

            convert_char(res)
        })
        .sum::<usize>()
}

fn part2(data: Vec<Vec<char>>) -> usize {
    data.chunks(3)
        .map(|set| {
            let mut res = ' ';
            for c in set[0].iter() {
                if set[1].contains(c) && set[2].contains(c) {
                    res = *c;
                    break;
                }
            }

            convert_char(res)
        })
        .sum::<usize>()
}

#[test]
fn test_part1() {
    assert_eq!(157, part1(get_data("test.txt")));
    assert_eq!(8493, part1(get_data("input.txt"))); // 12936
}

#[test]
fn test_part2() {
    assert_eq!(70, part2(get_data("test.txt")));
    assert_eq!(2552, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}
