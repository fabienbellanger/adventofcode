use std::{fs, io::SeekFrom, num::NonZeroUsize, os::macos::raw::stat};

#[derive(Debug)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug)]
struct Cube {
    x: usize,
    y: usize,
    state: State,
}

impl Cube {
    fn new(x: usize, y: usize, state: char) -> Self {
        let state = match state {
            '#' => State::Active,
            _ => State::Inactive,
        };
        Self {
            x,
            y,
            state,
        }
    }

    fn empty(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            state: State::Inactive,
        }
    }
}

#[derive(Debug)]
struct Level {
    z: isize,
    cubes: Vec<Cube>,
}

impl Level {
    fn new(z: isize, cubes: Vec<Cube>) -> Self {
        Self {
            z,
            cubes,
        }
    }
}

#[derive(Debug)]
struct PocketDimension {
    levels: Vec<Level>,
    height: usize,
}

impl PocketDimension {
    fn new(level: Level) -> Self {
        Self {
            height: 1,
            levels: vec![level],
        }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
}

fn part1(initial_level: Level) -> usize {
    let mut pocket_dimension = PocketDimension::new(initial_level);
    dbg!(&pocket_dimension);


    0
}

fn get_data(file: &str) -> Level {
    let cubes: Vec<Cube> = fs::read_to_string(file).expect("Cannot read file")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .trim()
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    Cube::new(x, y, c)
                })
                .collect::<Vec<Cube>>()
        })
        .collect();

    Level::new(0, cubes)
}

#[test]
fn test_part1() {
    assert_eq!(112, part1(get_data("test.txt")));
    // assert_eq!(, part1(get_data("input.txt")));
}
