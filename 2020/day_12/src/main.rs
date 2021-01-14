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

    fn manhattan_distance(&self) -> isize {
        self.north_south + self.east_west
    }

    fn change_direction(&mut self, angle: isize) {
        match self.direction {
            Action::North => match angle {
                90 | -270 => self.direction = Action::East,
                180 | -180 => self.direction = Action::South,
                -90 | 270 => self.direction = Action::West,
                _ => self.direction = Action::North,
            },
            Action::South => match angle {
                90 | -270 => self.direction = Action::West,
                180 | -180 => self.direction = Action::North,
                -90 | 270 => self.direction = Action::East,
                _ => self.direction = Action::South,
            },
            Action::East => match angle {
                90 | -270 => self.direction = Action::South,
                180 | -180 => self.direction = Action::West,
                -90 | 270 => self.direction = Action::North,
                _ => self.direction = Action::East,
            },
            Action::West => match angle {
                90 | -270 => self.direction = Action::North,
                180 | -180 => self.direction = Action::East,
                -90 | 270 => self.direction = Action::South,
                _ => self.direction = Action::West,
            },
            _ => (),
        };
    }

    fn move_forward(&mut self, direction: Option<&Action>, value: isize) {
        let direction = match direction {
            Some(d) => d,
            None => &self.direction,
        };
        match direction {
            Action::North => {
                self.north_south -= value;
            }
            Action::South => {
                self.north_south += value;
            }
            Action::East => {
                self.east_west += value;
            }
            Action::West => {
                self.east_west -= value;
            }
            _ => (),
        };
    }
}

fn main() {
    println!("Part 1 result: {}", part1(&get_data()));
    // println!("Part 2 result: {}", part2(get_data()));
}

fn part1(instructions: &[Instruction]) -> isize {
    let mut distance = Distance::new();

    for instruction in instructions {
        match instruction.action {
            Action::North | Action::South | Action::East | Action::West => {
                distance.move_forward(Some(&instruction.action), instruction.value);
            }
            Action::Left => {
                distance.change_direction(-instruction.value);
            }
            Action::Right => {
                distance.change_direction(instruction.value);
            }
            Action::Forward => {
                distance.move_forward(None, instruction.value);
            }
        };
    }

    distance.manhattan_distance()
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
            let (action, value) = line.trim().split_at(1);

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
            let (action, value) = line.trim().split_at(1);

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
