use std::{collections::HashMap, fs};

const SIZE: isize = 10;
const STEPS: u8 = 100;

fn main() {
    println!("Part 1 result: {}", part1(&mut get_data("input.txt")));
    println!("Part 2 result: {}", part2(&mut get_data("input.txt")));
}

fn flash(data: &mut HashMap<(isize, isize), isize>, row: isize, col: isize) -> usize {
    let mut flashes = 1;
    let directions: Vec<(isize, isize)> = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
    ];

    for dir in directions {
        if let Some(v) = data.get_mut(&(row + dir.0, col + dir.1)) {
            *v += 1;
            if *v == 10 {
                flashes += flash(data, row + dir.0, col + dir.1);
            }
        }
    }

    flashes
}

fn part1(data: &mut HashMap<(isize, isize), isize>) -> usize {
    let mut flashes = 0;

    for _ in 0..STEPS {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if let Some(v) = data.get_mut(&(row, col)) {
                    *v += 1;
                    if *v == 10 {
                        flashes += flash(data, row, col);
                    }
                }
            }
        }

        for row in 0..SIZE {
            for col in 0..SIZE {
                if let Some(v) = data.get_mut(&(row, col)) {
                    if *v >= 10 {
                        *v = 0;
                    }
                }
            }
        }
    }

    flashes
}

fn part2(data: &mut HashMap<(isize, isize), isize>) -> usize {
    let mut step = 0;

    loop {
        step += 1;
        for row in 0..SIZE {
            for col in 0..SIZE {
                if let Some(v) = data.get_mut(&(row, col)) {
                    *v += 1;
                    if *v == 10 {
                        flash(data, row, col);
                    }
                }
            }
        }

        let mut count_flashes = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                if let Some(v) = data.get_mut(&(row, col)) {
                    if *v >= 10 {
                        *v = 0;
                        count_flashes += 1;
                    }
                }
            }
        }

        if count_flashes == SIZE * SIZE {
            return step;
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(1656, part1(&mut get_data("test.txt")));
    assert_eq!(1729, part1(&mut get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(195, part2(&mut get_data("test.txt")));
    assert_eq!(237, part2(&mut get_data("input.txt")));
}

fn get_data(file: &str) -> HashMap<(isize, isize), isize> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, v)| {
                    (
                        (row as isize, col as isize),
                        v.to_digit(10).unwrap() as isize,
                    )
                })
                .collect::<HashMap<(isize, isize), isize>>()
        })
        .flatten()
        .collect()
}
