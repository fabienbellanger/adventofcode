use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Move(Direction),
    TurnLeft,  // 90, 180 or 270
    TurnRight, // 90, 180 or 270
    Forward,
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct Instruction {
    action: Action,
    value: isize,
}

impl Instruction {
    fn new(action: &str, value: &str) -> Self {
        let action = match action {
            "N" => Action::Move(Direction::North),
            "S" => Action::Move(Direction::South),
            "E" => Action::Move(Direction::East),
            "W" => Action::Move(Direction::West),
            "L" => Action::TurnLeft,
            "R" => Action::TurnRight,
            "F" => Action::Forward,
            _ => panic!("Invalid action {}", action),
        };

        Self {
            action,
            value: value.parse().expect("Invalid value"),
        }
    }
}

#[derive(Debug, Clone)]
struct Ship {
    position: Position,
    direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: Position { north: 0, east: 0 },
            direction: Direction::East,
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.position.north.abs() + self.position.east.abs()
    }

    fn change_direction(&mut self, angle: isize) {
        match self.direction {
            Direction::North => match angle {
                90 | -270 => self.direction = Direction::East,
                180 | -180 => self.direction = Direction::South,
                -90 | 270 => self.direction = Direction::West,
                _ => self.direction = Direction::North,
            },
            Direction::South => match angle {
                90 | -270 => self.direction = Direction::West,
                180 | -180 => self.direction = Direction::North,
                -90 | 270 => self.direction = Direction::East,
                _ => self.direction = Direction::South,
            },
            Direction::East => match angle {
                90 | -270 => self.direction = Direction::South,
                180 | -180 => self.direction = Direction::West,
                -90 | 270 => self.direction = Direction::North,
                _ => self.direction = Direction::East,
            },
            Direction::West => match angle {
                90 | -270 => self.direction = Direction::North,
                180 | -180 => self.direction = Direction::East,
                -90 | 270 => self.direction = Direction::South,
                _ => self.direction = Direction::West,
            },
        };
    }

    fn move_forward(&mut self, direction: Option<&Direction>, value: isize) {
        let direction = match direction {
            Some(d) => d,
            None => &self.direction,
        };
        match direction {
            Direction::North => {
                self.position.north += value;
            }
            Direction::South => {
                self.position.north -= value;
            }
            Direction::East => {
                self.position.east += value;
            }
            Direction::West => {
                self.position.east -= value;
            }
        };
    }

    fn forward(&mut self, value: isize, waypoint: &Position) {
        self.position.north += waypoint.north * value;
        self.position.east += waypoint.east * value;
    }
}

#[derive(Debug, Clone)]
struct Position {
    north: isize,
    east: isize,
}

impl Position {
    fn new() -> Self {
        Self { north: 1, east: 10 }
    }

    fn rotate(&mut self, angle: isize) {
        let waypoint = self.clone();
        match angle {
            90 | -270 => {
                self.north = -waypoint.east;
                self.east = waypoint.north;
            }
            180 | -180 => {
                self.north = -waypoint.north;
                self.east = -waypoint.east;
            }
            -90 | 270 => {
                self.north = waypoint.east;
                self.east = -waypoint.north;
            }
            _ => (),
        };
    }

    fn forward(&mut self, direction: &Direction, value: isize) {
        match direction {
            Direction::North => self.north += value,
            Direction::East => self.east += value,
            Direction::South => self.north -= value,
            Direction::West => self.east -= value,
        };
    }
}

fn main() {
    println!("Part 1 result: {}", part1(&get_data()));
    println!("Part 2 result: {}", part2(&get_data()));
}

fn part1(instructions: &[Instruction]) -> isize {
    let mut ship = Ship::new();

    for instruction in instructions {
        match &instruction.action {
            Action::Move(direction) => {
                ship.move_forward(Some(&direction), instruction.value);
            }
            Action::TurnLeft => {
                ship.change_direction(-instruction.value);
            }
            Action::TurnRight => {
                ship.change_direction(instruction.value);
            }
            Action::Forward => {
                ship.move_forward(None, instruction.value);
            }
        };
    }

    ship.manhattan_distance()
}

fn part2(instructions: &[Instruction]) -> isize {
    let mut ship = Ship::new();
    let mut waypoint = Position::new();

    for instruction in instructions {
        match &instruction.action {
            Action::Move(direction) => {
                waypoint.forward(direction, instruction.value);
            }
            Action::TurnLeft => {
                waypoint.rotate(-instruction.value);
            }
            Action::TurnRight => {
                waypoint.rotate(instruction.value);
            }
            Action::Forward => {
                ship.forward(instruction.value, &waypoint);
            }
        };
    }

    ship.manhattan_distance()
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

#[test]
fn test_part2() {
    assert_eq!(286, part2(&_get_data_test()));
    assert_eq!(45763, part2(&get_data()));
}
