use std::collections::HashMap;
use std::fmt::Formatter;
use std::{fmt, fs};
use utils::point::Point;

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

#[derive(Debug, Clone)]
struct Grid {
    rocks: HashMap<Point, Rock>,
    items: Vec<Vec<Item>>,
    width: usize,
    height: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut lines = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                // if let Some(rock) = self.rocks.get(&Point::new(x as isize, y as isize)) {
                //     match rock {
                //         Rock::Rounded => lines.push('O'),
                //         Rock::CubeShaped => lines.push('#'),
                //     }
                // } else {
                //     lines.push('.');
                // }
                match self.items[y][x] {
                    Item::Empty => lines.push('.'),
                    Item::Rounded => lines.push('0'),
                    Item::CubeShaped => lines.push('#'),
                }
            }
            lines.push('\n');
        }
        write!(f, "{lines}")
    }
}

impl Grid {
    fn total_load(&self) -> isize {
        self.rocks
            .iter()
            .filter_map(|(point, rock)| {
                if *rock == Rock::CubeShaped {
                    return None;
                }

                Some(self.height as isize - point.y)
            })
            .sum()
    }

    fn total_load_items(&self) -> isize {
        let mut total = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.items[y][x] == Item::Rounded {
                    total += self.height as isize - y as isize;
                }
            }
        }

        total
    }

    fn move_rocks_north(&mut self) {
        let current = self.rocks.clone();

        for x in 0..self.width {
            let mut rocks = current
                .iter()
                .filter(|&(point, _rock)| point.x == x as isize)
                .collect::<Vec<_>>();
            rocks.sort_by_key(|&(point, _rock)| point.y);

            let mut blocked_at = -1;
            for (y, &(point, rock)) in rocks.iter().enumerate() {
                if *rock == Rock::CubeShaped {
                    blocked_at = point.y;
                    continue;
                }

                if point.y > blocked_at {
                    blocked_at += 1;

                    self.rocks.remove(point);
                    self.rocks.insert(Point::new(x as isize, blocked_at), Rock::Rounded);
                }
            }
        }
    }

    fn move_rocks_north_items(&mut self) {
        for x in 0..self.width {
            let mut blocked_at = -1;

            for y in 0..self.height {
                let item = self.items[y][x].clone();

                if item == Item::Empty {
                    continue;
                }

                if item == Item::CubeShaped {
                    blocked_at = y as isize;
                    continue;
                }

                if item == Item::Rounded && y as isize > blocked_at {
                    blocked_at += 1;

                    self.items[y][x] = Item::Empty;
                    self.items[blocked_at as usize][x] = Item::Rounded;
                }
            }
        }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(mut data: Grid) -> isize {
    println!("{}", &data);
    data.move_rocks_north_items();
    println!("{}", &data);
    data.total_load_items()
}

fn part2(mut data: Grid) -> isize {
    for _ in 0..1_000_000_000 {
        data.move_rocks_north_items();
        data.move_rocks_north_items();
        data.move_rocks_north_items();
        data.move_rocks_north_items();
    }
    data.total_load_items()
    // todo!()
}

fn parse_input(file: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;
    let mut items = Vec::new();

    let rocks = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {file}"))
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            if y == 0 {
                width = line.len();
            }
            height += 1;

            items.push(Vec::new());

            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    items[y].push(Item::from(c));

                    if c != '.' {
                        let point = Point::new(x as isize, y as isize);
                        let rock = Rock::from(c);

                        return Some((point, rock));
                    }

                    None
                })
                .collect::<HashMap<Point, Rock>>()
        })
        .collect::<HashMap<Point, Rock>>();

    Grid {
        rocks,
        items,
        width,
        height,
    }
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
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
