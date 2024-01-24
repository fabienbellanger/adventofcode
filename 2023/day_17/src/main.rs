use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct Map {
    crucibles: Vec<Vec<u8>>,
    size: usize,
}

impl Map {}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(map: Map) -> usize {
    dbg!(&map);
    todo!()
}

fn part2(map: Map) -> usize {
    todo!()
}

fn parse_input(file: &str) -> Map {
    let crucibles: Vec<Vec<u8>> = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {file}"))
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect())
        .collect();
    let size = crucibles.len();

    Map { crucibles, size }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(102, part1(parse_input(TEST)));
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
