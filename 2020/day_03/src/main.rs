use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(data: Vec<String>) -> usize {
    // Right 3, down 1.
    walk(&data, 3, 1)
}

fn part2(data: Vec<String>) -> usize {
    // Right 1, down 1.
    // Right 3, down 1.
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.

    walk(&data, 1, 1)
        * walk(&data, 3, 1)
        * walk(&data, 5, 1)
        * walk(&data, 7, 1)
        * walk(&data, 1, 2)
}

fn walk(data: &[String], right: usize, down: usize) -> usize {
    // "." => empty square
    // "#" => tree

    let height = data.len();
    let width = data.first().expect("Empty line").len();

    let mut valids = 0;
    let mut x = 0;
    let mut y = 0;
    while y < height {
        let line = &data[y];
        let c = line.chars().nth((right * x) % width);

        if let Some(cc) = c {
            if cc == '#' {
                valids += 1;
            }
        }

        y += down;
        x += 1;
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
    assert_eq!(6708199680, part2(get_data()));
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
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}
