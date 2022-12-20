use std::collections::{HashMap, HashSet};
use std::{fmt, fs};

#[derive(Debug, Copy, Clone)]
enum Cell {
    Start,
    End,
    Square(u8),
}

impl Cell {
    fn elevation(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Square(e) => e,
        }
    }
}

struct CellRecord {
    prev: Option<GridCoord>,
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
    visited: HashMap<GridCoord, CellRecord>,
    current: HashSet<GridCoord>,
    num_steps: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut width = 0;
        let height = input.lines().count();
        let data = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                if i == 0 {
                    width = line.trim().len();
                }

                line.chars()
                    .map(|c| match c {
                        'S' => Cell::Start,
                        'E' => Cell::End,
                        'a'..='z' => Cell::Square(c as u8 - b'a'),
                        _ => panic!("invalid character: {c}"),
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect();

        Self {
            width,
            height,
            data,
            current: Default::default(),
            visited: Default::default(),
            num_steps: 0,
        }
    }
}

impl Grid {
    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn cell(&self, coord: GridCoord) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }

    fn walkable_neighbors(&self, coord: GridCoord) -> impl Iterator<Item = GridCoord> + '_ {
        let current_elevation = self.cell(coord).unwrap().elevation();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        directions.into_iter().filter_map(move |(dx, dy)| {
            Some(GridCoord {
                x: coord.x.checked_add_signed(dx)?,
                y: coord.y.checked_add_signed(dy)?,
            })
            .filter(|&coord| self.in_bounds(coord))
            .filter(|&coord| {
                let next_elevation = self.cell(coord).unwrap().elevation();
                next_elevation <= current_elevation + 1
            })
        })
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "---- {}x{} ----", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cell((x, y).into()).unwrap();
                let c = match cell {
                    Cell::Start => 'S',
                    Cell::End => 'E',
                    Cell::Square(elevation) => (b'a' + elevation) as char,
                };
                write!(f, "{c} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Grid) -> usize {
    println!("{data:?}");

    1
}

// fn part2(data: Grid) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(0, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Grid {
    Grid::parse(fs::read_to_string(file).expect("Cannot read the file input.txt").trim())
}
