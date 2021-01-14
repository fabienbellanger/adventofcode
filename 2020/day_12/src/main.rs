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
    north_south: isize, // North < 0, South > 0
    east_west: isize,   // West < 0,  East > 0
    direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Self {
            north_south: 0,
            east_west: 0,
            direction: Direction::East,
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.north_south + self.east_west
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
                self.north_south -= value;
            }
            Direction::South => {
                self.north_south += value;
            }
            Direction::East => {
                self.east_west += value;
            }
            Direction::West => {
                self.east_west -= value;
            }
        };
    }

    fn forward(&mut self, value: isize, waypoint: &WayPoint) {
        self.north_south += (waypoint.south - waypoint.north) * value;
        self.east_west += (waypoint.east - waypoint.west) * value;
    }
}

#[derive(Debug, Clone)]
struct WayPoint {
    north: isize,
    east: isize,
    south: isize,
    west: isize,
}

impl WayPoint {
    fn new() -> Self {
        Self {
            north: 1,
            east: 10,
            south: 0,
            west: 0,
        }
    }

    fn rotate(&mut self, angle: isize) {
        let waypoint = self.clone();
        match angle {
            90 | -270 => {
                self.north = waypoint.west;
                self.east = waypoint.north;
                self.south = waypoint.east;
                self.west = waypoint.south;
            },
            180 | -180 => {
                self.north = waypoint.south;
                self.east = waypoint.west;
                self.south = waypoint.north;
                self.west = waypoint.east;
            },
            -90 | 270 => {
                self.north = waypoint.east;
                self.east = waypoint.south;
                self.south = waypoint.west;
                self.west = waypoint.north;
            },
            _ => (),
        };
    }

    fn forward(&mut self, direction: &Direction, value: isize) {
        match direction {
            Direction::North => self.north += value,
            Direction::East => self.east += value,
            Direction::South => self.south += value,
            Direction::West => self.west += value,
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
    let mut waypoint = WayPoint::new();

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
        // dbg!(&ship, &waypoint);
        println!("{:?} {}  ---  ship: {}, {}  ---  waypoint: {}, {}, {}, {}",
            instruction.action,
            instruction.value,
            ship.north_south, 
            ship.east_west,
            waypoint.north,
            waypoint.east,
            waypoint.south,
            waypoint.west)
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
    // assert_eq!(2032, part2(get_data()));
}
