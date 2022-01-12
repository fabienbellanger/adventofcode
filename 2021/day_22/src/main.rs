use std::fs;

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone)]
struct Step {
    turn_on: bool,
    min_point: Point,
    max_point: Point,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(steps: Vec<Step>) -> usize {
    0
}

fn part2(steps: Vec<Step>) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(39, part1(get_data("test.txt")));
    assert_eq!(590784, part1(get_data("test_big.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(0, part2(get_data("test.txt")));
    // assert_eq!(0, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Step> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|line| {
            let (action, coords) = line.trim().split_once(' ').unwrap();

            let mut axis = coords.trim().split(',');
            let (x_min, x_max) = axis.next().unwrap().trim_start_matches("x=").split_once("..").unwrap();
            let (y_min, y_max) = axis.next().unwrap().trim_start_matches("y=").split_once("..").unwrap();
            let (z_min, z_max) = axis.next().unwrap().trim_start_matches("z=").split_once("..").unwrap();

            Step {
                turn_on: action == "on",
                min_point: Point {
                    x: x_min.parse().unwrap(),
                    y: y_min.parse().unwrap(),
                    z: z_min.parse().unwrap(),
                },
                max_point: Point {
                    x: x_max.parse().unwrap(),
                    y: y_max.parse().unwrap(),
                    z: z_max.parse().unwrap(),
                },
            }
        })
        .collect()
}
