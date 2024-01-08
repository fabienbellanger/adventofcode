use std::fs;

const INPUT: &str = "input.txt";

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn hash_string(s: &str) -> usize {
    s.chars().fold(0, |acc, c| {
        let mut h = acc + (c as u8) as usize;
        h *= 17;
        h %= 256;

        h
    })
}

fn part1(data: Vec<String>) -> usize {
    data.iter().map(|s| hash_string(s)).sum()
}

fn part2(data: Vec<String>) -> usize {
    todo!()
}

fn parse_input(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {file}"))
        .trim()
        .split(',')
        .map(|part| part.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(1_320, part1(parse_input(TEST)));
        assert_eq!(519_041, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(145, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
