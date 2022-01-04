use std::{cmp::Ordering, fs};

#[derive(Debug)]
struct Position {
    x: isize,
    y: isize,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn reached_target(target: &(Position, Position), speed: (isize, isize)) -> Option<isize> {
    let mut position = Position { x: speed.0, y: speed.1 };
    let mut reached =
        position.x >= target.0.x && position.x <= target.1.x && position.y <= target.0.y && position.y >= target.1.y;
    let mut speed = speed;
    let mut max_height = speed.1;

    while !(reached || position.x > target.1.x || position.y < target.1.y) {
        speed.0 += match speed.0.cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        speed.1 -= 1;
        position = Position {
            x: position.x + speed.0,
            y: position.y + speed.1,
        };

        if position.y > max_height {
            max_height = position.y;
        }

        if position.x >= target.0.x && position.x <= target.1.x && position.y <= target.0.y && position.y >= target.1.y
        {
            reached = true;
        }
    }

    match reached {
        true => Some(max_height),
        false => None,
    }
}

fn solve(target: (Position, Position)) -> (isize, usize) {
    let mut number = 0;
    let mut max_height = 0;

    for x in 1..=target.1.x {
        for y in target.1.y..=-target.1.y {
            if let Some(height) = reached_target(&target, (x, y)) {
                number += 1;

                if height > max_height {
                    max_height = height;
                }
            }
        }
    }

    (max_height, number)
}

fn part1(target: (Position, Position)) -> isize {
    let (h, _) = solve(target);

    h
}

fn part2(target: (Position, Position)) -> usize {
    let (_, n) = solve(target);

    n
}

#[test]
fn test_part1() {
    assert_eq!(45, part1(get_data("test.txt")));
    assert_eq!(3003, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(112, part2(get_data("test.txt")));
    assert_eq!(940, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (Position, Position) {
    // target area: x=20..30, y=-10..-5
    let data = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .chars()
        .skip(13)
        .collect::<String>();

    let data = data.split_once(", ").unwrap();
    let part1 = data.0.chars().skip(2).collect::<String>();
    let part2 = data.1.chars().skip(2).collect::<String>();

    let (x1, x2) = part1.split_once("..").unwrap();
    let (y1, y2) = part2.split_once("..").unwrap();

    let (x1, x2) = (x1.parse().unwrap(), x2.parse().unwrap());
    let (y1, y2) = (y1.parse().unwrap(), y2.parse().unwrap());

    (
        Position {
            x: std::cmp::min(x1, x2),
            y: std::cmp::max(y1, y2),
        },
        Position {
            x: std::cmp::max(x1, x2),
            y: std::cmp::min(y1, y2),
        },
    )
}
