use std::fmt::Formatter;
use std::ops::Add;
use std::{collections::HashSet, fmt, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>,
}

impl Path {
    fn parse(line: &str) -> Self {
        let points = line
            .split(" -> ")
            .map(|p| {
                let (x, y) = p.split_once(',').unwrap();
                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect::<Vec<_>>();
        Self { points }
    }
}

#[derive(Debug)]
struct Grid {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    origin: Point,
    current: Point,
    rocks: HashSet<Point>,
    sands: HashSet<Point>,
}

impl Grid {
    fn parse(data: &str) -> Self {
        let paths = data
            .trim()
            .lines()
            .map(|line| Path::parse(line.trim()))
            .collect::<Vec<_>>();

        let mut grid = Self {
            x_min: isize::MAX,
            x_max: isize::MIN,
            y_min: isize::MAX,
            y_max: isize::MIN,
            origin: Point::from((500, 0)),
            current: Point::from((500, 0)),
            rocks: HashSet::new(),
            sands: HashSet::new(),
        };
        grid.rocks_from_paths(paths);

        grid
    }

    fn in_bounds(&self, point: Point) -> bool {
        point.x <= self.x_max && point.x >= self.x_min && point.y >= self.y_min && point.y <= self.y_max
    }

    fn rocks_from_paths(&mut self, paths: Vec<Path>) {
        for path in paths {
            let points = path.points;
            for pair in points.windows(2) {
                let p1 = pair.get(0).unwrap();
                let p2 = pair.get(1).unwrap();

                let (range_x, range_y) = match (p1, p2) {
                    (p1, p2) if p1.x == p2.x && p1.y < p2.y => (p1.x..=p2.x, p1.y..=p2.y),
                    (p1, p2) if p1.x == p2.x && p1.y >= p2.y => (p1.x..=p2.x, p2.y..=p1.y),
                    (p1, p2) if p1.y == p2.y && p1.x < p2.x => (p1.x..=p2.x, p1.y..=p2.y),
                    (p1, p2) if p1.y == p2.y && p1.x >= p2.x => (p2.x..=p1.x, p1.y..=p2.y),
                    _ => panic!(),
                };

                for x in range_x {
                    for y in range_y.clone() {
                        self.rocks.insert(Point::from((x, y)));
                        if x < self.x_min {
                            self.x_min = x
                        };
                        if x > self.x_max {
                            self.x_max = x
                        };
                        if y < self.y_min {
                            self.y_min = y
                        };
                        if y > self.y_max {
                            self.y_max = y
                        };
                    }
                }
            }
        }

        if self.y_min > self.origin.y {
            self.y_min = self.origin.y;
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in self.y_min..=self.y_max + 2 {
            for x in self.x_min - 1..=self.x_max + 1 {
                let point = Point::from((x, y));
                write!(
                    f,
                    "{}",
                    if self.rocks.contains(&point) {
                        "#"
                    } else if self.sands.contains(&point) {
                        "o"
                    } else if point == self.origin {
                        "+"
                    } else if point == self.current {
                        "x"
                    } else {
                        "."
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("day_14/input.txt")));
    println!("Part 2 result: {}", part2(get_data("day_14/input.txt")));
}

fn part1(mut data: Grid) -> usize {
    loop {
        // Stop if out of bounds
        if !data.in_bounds(data.current) {
            break;
        }

        let moves = [
            data.current + Point::from((0, 1)),
            data.current + Point::from((-1, 1)),
            data.current + Point::from((1, 1)),
        ];

        // Can move?
        if let Some(position) = moves
            .into_iter()
            .find(|p| !data.rocks.contains(p) && !data.sands.contains(p))
        {
            data.current = position;
            continue;
        }

        // Sand is stopped
        data.sands.insert(data.current);
        data.current = data.origin;
    }

    data.sands.len()
}

fn part2(mut data: Grid) -> usize {
    let floor_x_min = data.x_min - 10 * (data.origin.x - data.x_min) + 1;
    let floor_x_max = data.x_max + 10 * (data.x_max - data.origin.x) + 1;
    for x in floor_x_min..=floor_x_max {
        data.rocks.insert(Point::from((x, data.y_max + 2)));
    }

    loop {
        // Stop if sand is stopped at origin
        if data.sands.contains(&data.origin) {
            break;
        }

        let moves = [
            data.current + Point::from((0, 1)),
            data.current + Point::from((-1, 1)),
            data.current + Point::from((1, 1)),
        ];

        // Can move?
        if let Some(position) = moves
            .into_iter()
            .find(|p| !data.rocks.contains(p) && !data.sands.contains(p))
        {
            data.current = position;
            continue;
        }

        // Sand is stopped
        data.sands.insert(data.current);
        data.current = data.origin;
    }

    data.sands.len()
}

#[test]
fn test_part1() {
    assert_eq!(24, part1(get_data("test.txt")));
    assert_eq!(885, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(93, part2(get_data("test.txt")));
    assert_eq!(28691, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Grid {
    let data = fs::read_to_string(file).expect("Cannot read the file input.txt");
    Grid::parse(&data)
}
