use std::fs;

/*
0: abcefg (6)
1: cf (2)
2: acdeg (5)
3: acdfg (5)
4: bcdf (4)
5: abdfg (5)
6: abdefg (6)
7: acf (3)
8: abcdefg (7)
9: abcdfg (6)
*/

fn main() {
    // println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1() -> usize {
    0
}

fn part2() -> usize {
    0
}

#[test]
fn test_part1() {
    // assert_eq!(37, part1(get_data("test.txt")));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
}
