use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

// See https://www.programiz.com/dsa/dijkstra-algorithm
fn part1(data: (HashMap<(isize, isize), usize>, usize, usize)) -> usize {
    let (y_max, x_max) = (data.1, data.2);
    let data = data.0;
    dbg!(&data, (y_max, x_max));

    0
}

fn part2() -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(40, part1(get_data("test.txt")));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (HashMap<(isize, isize), usize>, usize, usize) {
    let (mut y_max, mut x_max) = (0, 0);

    let data = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, v)| {
                    if row > y_max {
                        y_max = row;
                    }

                    if col > x_max {
                        x_max = col;
                    }

                    ((row as isize, col as isize), v.to_digit(10).unwrap() as usize)
                })
                .collect::<HashMap<(isize, isize), usize>>()
        })
        .flatten()
        .collect();

    (data, y_max, x_max)
}
