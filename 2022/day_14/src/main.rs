use std::{fs, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Path {
    points: Vec<Point>
}

impl Path {
    fn parse(line: &str) -> Self {
        let points = line.split(" -> ")
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

        Self {
            rocks: Self::rocks_from_paths(paths),
            sands: HashSet::new(),
        }
    }

    fn rocks_from_paths(paths: Vec<Path>) -> HashSet<Point> {
        dbg!(&paths);

        HashSet::new()
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Grid) -> usize {
    println!("Grid={:?}", &data);

    0
}

// fn part2(data: Grid) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(24, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Grid {
    let data = fs::read_to_string(file)
        .expect("Cannot read the file input.txt");
    Grid::parse(&data)
}
