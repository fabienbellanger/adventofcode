use std::collections::HashSet;
use std::fmt::Formatter;
use std::{fmt, fs};

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

impl Point {
    fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn tuning_frequency(&self) -> isize {
        self.x * 4_000_000 + self.y
    }
}

#[derive(Debug)]
struct Record {
    sensor: Point,
    beacon: Point,
    distance: usize,
}

impl Record {
    fn parse(line: &str) -> Self {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let line = line.trim().strip_prefix("Sensor at ").unwrap();
        let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();

        // Sensor
        let (sensor_x, sensor_y) = sensor.split_once(", ").unwrap();
        let sensor_x = sensor_x.strip_prefix("x=").unwrap().parse().unwrap();
        let sensor_y = sensor_y.strip_prefix("y=").unwrap().parse().unwrap();
        let sensor = Point::from((sensor_x, sensor_y));

        // Beacon
        let (beacon_x, beacon_y) = beacon.split_once(", ").unwrap();
        let beacon_x = beacon_x.strip_prefix("x=").unwrap().parse().unwrap();
        let beacon_y = beacon_y.strip_prefix("y=").unwrap().parse().unwrap();
        let beacon = Point::from((beacon_x, beacon_y));

        // Distance
        let distance = sensor.manhattan_distance(&beacon);

        Self {
            sensor,
            beacon,
            distance,
        }
    }
}

#[derive(Debug)]
struct Grid {
    records: Vec<Record>,
}

impl Grid {
    fn parse(data: &str) -> Self {
        let records = data.trim().lines().map(Record::parse).collect();

        Self { records }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("day_15/input.txt"), 2_000_000));
    println!("Part 2 result: {}", part2(get_data("day_15/input.txt"), 0, 4_000_000));
}

fn part1(grid: Grid, line: isize) -> usize {
    let records = grid.records;

    // Ranges list of valid sensors
    let mut ranges = vec![];
    for record in &records {
        let distance_line = line.abs_diff(record.sensor.y);

        if distance_line > record.distance {
            continue;
        }

        let length = record.distance - distance_line;
        let start = record.sensor.x - length as isize;
        let end = record.sensor.x + length as isize;
        ranges.push(start..=end);
    }
    ranges.sort_by_key(|r| *r.start());

    // Count without ranges intersections
    let mut possible_positions = 0;
    let mut last = isize::MIN;
    for r in ranges {
        for i in r {
            if i > last {
                last = i;
                possible_positions += 1;
            }
        }
    }

    // Find beacons in line
    let beacon_positions = records
        .into_iter()
        .filter(|r| r.beacon.y == line)
        .map(|r| r.beacon.x)
        .collect::<HashSet<_>>()
        .len();

    possible_positions - beacon_positions
}

fn part2(grid: Grid, start: isize, end: isize) -> usize {
    0
}

#[test]
fn test_manhattan_distance() {
    let p1 = Point::from((8, 7));
    let p2 = Point::from((2, 10));

    assert_eq!(9, p1.manhattan_distance(&p2));
}

#[test]
fn test_part1() {
    assert_eq!(26, part1(get_data("test.txt"), 10));
    assert_eq!(5716881, part1(get_data("input.txt"), 2_000_000));
}

#[test]
fn test_part2() {
    assert_eq!(56000011, part2(get_data("test.txt"), 0, 20));
    // assert_eq!(0, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Grid {
    let data = fs::read_to_string(file).expect("Cannot read the file input.txt");

    Grid::parse(&data)
}
