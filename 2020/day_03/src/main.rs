use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(data: Vec<String>) -> usize {
    // "." => empty square
    // "#" => tree

    let right = 3;
    let height = data.len();
    let width = data.first().expect("Empty line").len();

    let mut valids = 0;
    for i in 0..height {
        let line = &data[i];
        let c = line.chars().nth((right * i) % width);
        
        if let Some(cc) = c {
            if cc == '#' {
                valids += 1;
            }
        }
    }

    valids
}

fn part2(data: Vec<String>) -> usize {
    // "." => empty square
    // "#" => tree

    let right = 3;
    let height = data.len();
    let width = data.first().expect("Empty line").len();

    let mut valids = 0;
    for i in 0..height {
        let line = &data[i];
        let c = line.chars().nth((right * i) % width);
        
        if let Some(cc) = c {
            if cc == '#' {
                valids += 1;
            }
        }
    }

    valids
}

#[test]
fn test_part1() {
    assert_eq!(7, part1(_get_data_test()));
    assert_eq!(216, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(336, part2(_get_data_test()));
    // assert_eq!(216, part2(get_data()));
}

fn get_data() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

fn _get_data_test() -> Vec<String> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}
