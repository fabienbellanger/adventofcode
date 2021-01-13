use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Action {
    North,
    South,
    East,
    West,
    Left,  // 90, 180 or 270
    Right, // 90, 180 or 270
    Forward,
}

#[derive(Debug, Clone)]
struct Instruction {
    action: Action,
    value: isize,
}

impl Instruction {
    fn new(action: &str, value: &str) -> Self {
        let action = match action {
            "N" => Action::North,
            "S" => Action::South,
            "E" => Action::East,
            "W" => Action::West,
            "L" => Action::Left,
            "R" => Action::Right,
            "F" => Action::Forward,
            _ => panic!("Invalid action"),
        };

        Self {
            action,
            value: value.parse().expect("Invalid value"),
        }
    }
}

#[derive(Debug, Clone)]
struct Distance {
    north_south: isize, // North < 0, South > 0
    east_west: isize,   // West < 0,  East > 0
    direction: Action,
}

impl Distance {
    fn new() -> Self {
        Self {
            north_south: 0,
            east_west: 0,
            direction: Action::East,
        }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(&get_data()));
    // println!("Part 2 result: {}", part2(get_data()));
}

fn part1(instructions: &Vec<Instruction>) -> isize {
    let mut distance = Distance::new();

    for instruction in instructions {
        match instruction.action {
            Action::North | Action::South | Action::East | Action::West => {
                move_boat(&mut distance, &instruction.action, instruction.value);
            },
            Action::Left => {
                distance.direction = change_direction(&distance, -instruction.value);
            },
            Action::Right => {
                distance.direction = change_direction(&distance, instruction.value);
            },
            Action::Forward => {
                let direction= &(distance.clone()).direction;
                move_boat(&mut distance, direction, instruction.value);
            },
        };
    }

    distance.north_south + distance.east_west
}

fn move_boat(distance: &mut Distance, direction: &Action, value: isize) {
    match direction {
        Action::North => {
            distance.north_south -= value;
        },
        Action::South => {
            distance.north_south += value;
        },
        Action::East => {
            distance.east_west += value;
        },
        Action::West => {
            distance.east_west -= value;
        },
        _ => (),
    };
}

fn change_direction(distance: &Distance, angle: isize) -> Action {
    match distance.direction {
        Action::North => {
            match angle {
                90 | -270 => Action::East,
                180 | -180 => Action::South,
                -90 | 270 => Action::West,
                _ => Action::North,
            }
        },
        Action::South => {
            match angle {
                90 | -270 => Action::West,
                180 | -180 => Action::North,
                -90 | 270 => Action::East,
                _ => Action::South,
            }
        },
        Action::East => {
            match angle {
                90 | -270 => Action::South,
                180 | -180 => Action::West,
                -90 | 270 => Action::North,
                _ => Action::East,
            }
        },
        Action::West => {
            match angle {
                90 | -270 => Action::North,
                180 | -180 => Action::East,
                -90 | 270 => Action::South,
                _ => Action::West,
            }
        },
        _ => distance.direction.clone(),
    }
}

fn part2() -> usize {
    0
}

fn get_data() -> Vec<Instruction> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| {
            let (action , value) = line.trim().split_at(1);

            Instruction::new(action, value)
        })
        .collect()
}

fn _get_data_test() -> Vec<Instruction> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|line| {
            let (action , value) = line.trim().split_at(1);

            Instruction::new(action, value)
        })
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(25, part1(&_get_data_test()));
    assert_eq!(759, part1(&get_data()));
}

// #[test]
// fn test_part2() {
//     assert_eq!(26, part2(_get_data_test()));
//     assert_eq!(2032, part2(get_data()));
// }
