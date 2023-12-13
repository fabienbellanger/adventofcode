#![allow(unused_variables)]
use std::fs;

const INPUT: &str = "input.txt";

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Vec<Vec<isize>>) -> isize {
    data.into_iter().map(|line| get_extrapolated_value(line, false)).sum()
}

// 5  10  13  16  21  30  45
//   5   3   3   5   9  15
//    -2   0   2   4   6
//       2   2   2   2
//         0   0   0
fn part2(data: Vec<Vec<isize>>) -> isize {
    data.into_iter().map(|line| get_extrapolated_value(line, true)).sum()
}

fn get_extrapolated_value(line: Vec<isize>, is_part2: bool) -> isize {
    let lines = process_line(line);
    let mut value = 0;

    for i in (0..(lines.len() - 1)).rev() {
        if is_part2 {
            let first = lines[i].first().unwrap();
            value = first - value;
        } else {
            let last = lines[i].last().unwrap();
            value += *last;
        }
    }

    value
}

fn process_line(line: Vec<isize>) -> Vec<Vec<isize>> {
    let mut result = vec![line.clone()];
    let mut current = line.clone();

    while !is_zero(&current) {
        let mut next = vec![];
        for i in 0..(current.len() - 1) {
            next.push(current[i + 1] - current[i]);
        }

        current = next;

        result.push(current.clone());
    }

    result
}

fn is_zero(line: &Vec<isize>) -> bool {
    line.iter().all(|n| *n == 0)
}

fn parse_input(file: &str) -> Vec<Vec<isize>> {
    fs::read_to_string(file)
        .expect(&format!("Cannot read the file {file}"))
        .trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|value| value.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_process_line() {
        let input = vec![0, 3, 6, 9, 12, 15];
        let expected = vec![vec![0, 3, 6, 9, 12, 15], vec![3, 3, 3, 3, 3], vec![0, 0, 0, 0]];
        assert_eq!(process_line(input), expected);

        let input = vec![10, 13, 16, 21, 30, 45];
        let expected = vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3, 3, 5, 9, 15],
            vec![0, 2, 4, 6],
            vec![2, 2, 2],
            vec![0, 0],
        ];
        assert_eq!(process_line(input), expected);
    }

    #[test]
    fn test_get_extrapolated_value() {
        let input = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(get_extrapolated_value(input, false), 18);

        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(get_extrapolated_value(input, false), 68);

        let input = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(get_extrapolated_value(input, true), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(114, part1(parse_input(TEST)));
        assert_eq!(1_581_679_977, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2(parse_input(TEST)));
        assert_eq!(889, part2(parse_input(INPUT)));
    }
}
