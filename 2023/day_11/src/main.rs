use std::fmt::Formatter;
use std::{fmt, fs};
use utils::point::Point;

const INPUT: &str = "input.txt";

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Point>,
    dimension: (usize, usize),
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut lines = String::new();
        for y in 0..self.dimension.0 {
            for x in 0..self.dimension.1 {
                if self.galaxies.contains(&Point::new(x as isize, y as isize)) {
                    lines.push('#');
                } else {
                    lines.push('.');
                }
            }
            lines.push('\n');
        }
        write!(f, "{lines}")
    }
}

impl Universe {
    fn new(galaxies: Vec<Point>) -> Self {
        let mut universe = Self {
            galaxies,
            dimension: (0, 0),
        };
        universe.update_dimension();

        universe
    }

    fn update_dimension(&mut self) {
        let mut max_x = 0;
        let mut max_y = 0;

        for galaxy in self.galaxies.iter() {
            if galaxy.x > max_x {
                max_x = galaxy.x;
            }
            if galaxy.y > max_y {
                max_y = galaxy.y;
            }
        }

        self.dimension = (max_x as usize + 1, max_y as usize + 1)
    }

    fn find_rows(&self) -> Vec<isize> {
        let mut rows = Vec::new();

        for x in 0..self.dimension.0 {
            let x = x as isize;
            let present = self.galaxies.iter().any(|g| g.x == x);
            if !present {
                rows.push(x);
            }
        }

        rows
    }

    fn find_cols(&self) -> Vec<isize> {
        let mut cols = Vec::new();

        for y in 0..self.dimension.1 {
            let y = y as isize;
            let present = self.galaxies.iter().any(|g| g.y == y);
            if !present {
                cols.push(y);
            }
        }

        cols
    }

    fn expand(&mut self, step: isize) {
        let rows = self.find_rows();
        let cols = self.find_cols();
        let old_galaxies = self.galaxies.clone();

        for col in cols {
            for (i, galaxy) in self.galaxies.iter_mut().enumerate() {
                if old_galaxies[i].y > col {
                    galaxy.y += step;
                }
            }
        }

        for row in rows {
            for (i, galaxy) in self.galaxies.iter_mut().enumerate() {
                if old_galaxies[i].x > row {
                    galaxy.x += step;
                }
            }
        }

        self.update_dimension();
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT), 1_000_000));
}

fn process(data: Vec<Point>, step: isize) -> usize {
    let mut universe = Universe::new(data);
    universe.expand(step);

    let mut s = 0;
    for (i, p) in universe.galaxies.iter().enumerate() {
        for (j, q) in universe.galaxies.iter().enumerate() {
            if i < j {
                s += p.manhattan_distance(&q);
            }
        }
    }

    s
}

fn part1(data: Vec<Point>) -> usize {
    process(data, 1)
}

fn part2(data: Vec<Point>, step: isize) -> usize {
    process(data, step - 1)
}

fn parse_input(file: &str) -> Vec<Point> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {file}"))
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| Point::new(x as isize, y as isize))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(374, part1(parse_input(TEST)));
        assert_eq!(9_608_724, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1_030, part2(parse_input(TEST), 10));
        assert_eq!(8_410, part2(parse_input(TEST), 100));
        assert_eq!(82_000_210, part2(parse_input(TEST), 1_000_000));
        assert_eq!(904_633_799_472, part2(parse_input(INPUT), 1_000_000));
    }
}
