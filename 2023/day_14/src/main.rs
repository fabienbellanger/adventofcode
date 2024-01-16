use std::fmt::Formatter;
use std::{fmt, fs};

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Rock {
    Rounded,
    CubeShaped,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::CubeShaped,
            'O' => Self::Rounded,
            v => panic!("invalid input: {v}"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Item {
    Rounded,
    CubeShaped,
    Empty,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::CubeShaped,
            'O' => Self::Rounded,
            '.' => Self::Empty,
            v => panic!("invalid input: {v}"),
        }
    }
}

impl From<Item> for char {
    fn from(value: Item) -> Self {
        match value {
            Item::CubeShaped => '#',
            Item::Rounded => 'O',
            Item::Empty => '.',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone)]
struct Grid {
    items: Vec<Vec<Item>>,
    width: usize,
    height: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut lines = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                lines.push(self.items[y][x].into());
            }
            lines.push('\n');
        }
        write!(f, "{lines}")
    }
}

impl Grid {
    fn items_to_string(&self) -> String {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| char::from(self.items[y][x])))
            .collect()
    }

    fn total_load_items(&self) -> isize {
        (0..self.height)
            .flat_map(|y| {
                (0..self.width)
                    .filter(move |&x| self.items[y][x] == Item::Rounded)
                    .map(move |_x| (self.height - y) as isize)
            })
            .sum()
    }

    fn move_rocks(&mut self, direction: Direction) {
        for i in 0..self.width {
            let mut blocked_at = match direction {
                Direction::North => -1,
                Direction::South => self.height as isize,
                Direction::West => -1,
                Direction::East => self.width as isize,
            };

            for j in 0..self.height {
                let (x, y) = match direction {
                    Direction::North => (i, j),
                    Direction::South => (i, self.height - j - 1),
                    Direction::West => (j, i),
                    Direction::East => (self.height - j - 1, i),
                };

                let item = self.items[y][x];

                if item == Item::Empty {
                    continue;
                }

                if item == Item::CubeShaped {
                    blocked_at = match direction {
                        Direction::North | Direction::South => y as isize,
                        Direction::West | Direction::East => x as isize,
                    };
                    continue;
                }

                if (item == Item::Rounded)
                    && ((direction == Direction::North && y as isize > blocked_at)
                        || (direction == Direction::South && y < blocked_at as usize)
                        || (direction == Direction::West && x as isize > blocked_at)
                        || (direction == Direction::East && x < blocked_at as usize))
                {
                    blocked_at += match direction {
                        Direction::North | Direction::West => 1,
                        Direction::South | Direction::East => -1,
                    };

                    self.items[y][x] = Item::Empty;

                    match direction {
                        Direction::North | Direction::South => {
                            self.items[blocked_at as usize][x] = Item::Rounded;
                        }
                        Direction::West | Direction::East => {
                            self.items[y][blocked_at as usize] = Item::Rounded;
                        }
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.move_rocks(Direction::North);
        self.move_rocks(Direction::West);
        self.move_rocks(Direction::South);
        self.move_rocks(Direction::East);
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(mut data: Grid) -> isize {
    data.move_rocks(Direction::North);
    data.total_load_items()
}

fn part2(mut data: Grid) -> isize {
    let mut cache = Vec::new();
    let mut cycle = 0;

    let mut data_copy = data.clone();
    let mut p = 0;
    for i in 0..1_000_000_000 {
        let items_str = data_copy.items_to_string();
        let pos = cache.iter().position(|s| *s == items_str);
        if let Some(pos) = pos {
            cycle = i - pos;
            p = pos;
            break;
        } else {
            cache.push(items_str);
        }

        data_copy.cycle();
    }

    //       <-cycle->
    // |oooo|#########|---------...--------|
    // 0    p     p + cycle          1_000_000_000
    let bound = ((1_000_000_000 - p) % cycle) + cycle;
    // println!("pos = {p}, cycle = {cycle}, bound = {bound}");
    for _ in 0..bound + p {
        data.cycle();
    }
    data.total_load_items()
}

fn parse_input(file: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;
    let mut items = Vec::new();

    let data = fs::read_to_string(file).unwrap_or_else(|_| panic!("Cannot read the file {file}"));

    for (y, line) in data.trim().lines().enumerate() {
        if y == 0 {
            width = line.len();
        }
        height += 1;

        items.push(Vec::new());

        for c in line.chars() {
            items[y].push(Item::from(c));
        }
    }

    Grid { items, width, height }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(136, part1(parse_input(TEST)));
        assert_eq!(112_773, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(64, part2(parse_input(TEST)));
        assert_eq!(98_894, part2(parse_input(INPUT)));
    }
}
