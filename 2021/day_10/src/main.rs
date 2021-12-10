use std::{collections::HashMap, fs};

/*
    ): 3 points.
    ]: 57 points.
    }: 1197 points.
    >: 25137 points.
*/

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn get_symbols() -> HashMap<char, (char, usize)> {
    let mut symbols: HashMap<char, (char, usize)> = HashMap::with_capacity(4);
    symbols.insert(')', ('(', 3));
    symbols.insert(']', ('[', 57));
    symbols.insert('}', ('{', 1_197));
    symbols.insert('>', ('<', 25_137));

    symbols
}

fn part1(data: Vec<Vec<char>>) -> usize {
    dbg!(&data);

    let symbols = get_symbols();
    println!("{:?}", &symbols);

    // 1. Collect errors in a HashMap<char, usize>
    // 2. Sum(nb_char * char_points)
    0
}

fn part2(_data: Vec<Vec<char>>) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(26397, part1(get_data("test.txt")));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|n| n.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
