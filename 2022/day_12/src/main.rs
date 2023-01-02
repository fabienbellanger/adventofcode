use std::collections::{HashMap, HashSet};
use std::{fmt, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
struct CellRecord {
    #[allow(dead_code)]
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

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn cell(&self, coord: GridCoord) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    fn walkable_neighbors(&self, coord: GridCoord, reverse: bool) -> impl Iterator<Item = GridCoord> + '_ {
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
                !reverse && next_elevation <= current_elevation + 1
                    || reverse && next_elevation >= current_elevation - 1
            })
        })
    }

    fn find_coord(&self, cell: Cell) -> GridCoord {
        let mut wanted_coord: Option<GridCoord> = None;
        for y in 0..self.height {
            for x in 0..self.width {
                let coord: GridCoord = (x, y).into();
                if &cell == self.cell(coord).unwrap() {
                    wanted_coord = Some(coord);
                    break;
                }
            }
        }
        wanted_coord.unwrap()
    }

    fn step(&mut self, reverse: bool) {
        if self.current.is_empty() {
            // Find start coordinate
            let start_coord = if reverse {
                self.find_coord(Cell::End)
            } else {
                self.find_coord(Cell::Start)
            };
            self.current.insert(start_coord);
            self.visited.insert(start_coord, CellRecord { prev: None });
            return;
        }

        let current = std::mem::take(&mut self.current);
        let mut next = HashSet::new();
        let mut visited = std::mem::take(&mut self.visited);

        for curr in current {
            for ncoord in self.walkable_neighbors(curr, reverse) {
                if visited.contains_key(&ncoord) {
                    // Don't visit it again!
                    continue;
                }
                visited.insert(ncoord, CellRecord { prev: Some(curr) });
                next.insert(ncoord);
            }
        }
        self.current = next;
        self.visited = visited;
        self.num_steps += 1;

        dbg!(&self.num_steps);
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
        writeln!(f, "Visited: {} ", self.num_steps)?;
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
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(mut data: Grid) -> usize {
    let cells_number = data.width * data.height;
    let end_cell = data.find_coord(Cell::End);

    while !(data.visited.len() == cells_number || data.visited.contains_key(&end_cell)) {
        data.step(false);
    }

    data.num_steps
}

fn part2(mut data: Grid) -> usize {
    let cells_number = data.width * data.height;

    'l: while !(data.visited.len() == cells_number) {
        for coord in &data.current {
            if data.cell(*coord).unwrap().elevation() == 0 {
                break 'l;
            }
        }

        data.step(true);
    }

    data.num_steps
}

#[test]
fn test_part1() {
    assert_eq!(31, part1(get_data("test.txt")));
    assert_eq!(383, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(29, part2(get_data("test.txt")));
    assert_eq!(377, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Grid {
    Grid::parse(fs::read_to_string(file).expect("Cannot read the file input.txt").trim())
}
