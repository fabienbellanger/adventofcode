use std::fs;

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    units: isize,
}

#[derive(Default)]
struct Position {
    depth: isize,
    horizontal: isize,
    aim: isize,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(lines: Vec<Movement>) -> isize {
    let mut position = Position::default();

    for line in lines {
        match line.direction {
            Direction::Forward => position.horizontal += line.units,
            Direction::Down => position.depth += line.units,
            Direction::Up => position.depth -= line.units,
        }
    }

    position.depth * position.horizontal
}

fn part2(lines: Vec<Movement>) -> isize {
    let mut position = Position::default();

    for line in lines {
        match line.direction {
            Direction::Forward => {
                position.horizontal += line.units;
                position.depth += position.aim * line.units;
            }
            Direction::Down => position.aim += line.units,
            Direction::Up => position.aim -= line.units,
        }
    }

    position.depth * position.horizontal
}

#[test]
fn test_part1() {
    assert_eq!(150, part1(_movements_test()));
    assert_eq!(1507611, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(900, part2(_movements_test()));
    assert_eq!(1880593125, part2(get_data()));
}

fn _movements_test() -> Vec<Movement> {
    vec![
        Movement {
            direction: Direction::Forward,
            units: 5,
        },
        Movement {
            direction: Direction::Down,
            units: 5,
        },
        Movement {
            direction: Direction::Forward,
            units: 8,
        },
        Movement {
            direction: Direction::Up,
            units: 3,
        },
        Movement {
            direction: Direction::Down,
            units: 8,
        },
        Movement {
            direction: Direction::Forward,
            units: 2,
        },
    ]
}

fn get_data() -> Vec<Movement> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let direction = match parts.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("invalid direction"),
            };
            let units = parts.next().unwrap().parse().unwrap();

            Movement { direction, units }
        })
        .collect()
}
