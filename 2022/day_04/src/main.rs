use std::{fs, ops::RangeInclusive};

type Section = RangeInclusive<u8>;
type Pair = (Section, Section);

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Pair>) -> usize {
    data.iter()
        .filter(|(s1, s2)| {
            s1.start() <= s2.start() && s1.end() >= s2.end() || 
            s2.start() <= s1.start() && s2.end() >= s1.end()
        })
        .count()
}

fn part2(data: Vec<Pair>) -> usize {
    data.iter()
        .filter(|(s1, s2)| {
            s2.start() >= s1.start() && s2.start() <= s1.end() || 
            s2.end() >= s1.start() && s2.end() <= s1.end() ||
            s1.start() >= s2.start() && s1.start() <= s2.end() || 
            s1.end() >= s2.start() && s1.end() <= s2.end()
        })
        .count()
}

#[test]
fn test_part1() {
    assert_eq!(2, part1(get_data("test.txt")));
    assert_eq!(498, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(4, part2(get_data("test.txt")));
    assert_eq!(0, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Pair> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| {
            let mut pair = line.split(',');
            let section_1 = pair.next().unwrap();
            let section_2 = pair.next().unwrap();

            let mut section_1 = section_1.split('-');
            let mut section_2 = section_2.split('-');

            let section_1_start = section_1.next().unwrap().parse::<u8>().unwrap();
            let section_1_end = section_1.next().unwrap().parse::<u8>().unwrap();

            let section_2_start = section_2.next().unwrap().parse::<u8>().unwrap();
            let section_2_end = section_2.next().unwrap().parse::<u8>().unwrap();

            ((section_1_start..=section_1_end), (section_2_start..=section_2_end))
        })
        .collect()
}
