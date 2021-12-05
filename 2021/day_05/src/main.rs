use std::fs;

#[derive(Default, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Default, Debug)]
struct Vents {
    lines: Vec<(Coordinates, Coordinates)>,
    x_max: usize,
    y_max: usize,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(vents: Vents) -> usize {
    let mut grid: Vec<Vec<u16>> = vec![vec![0; vents.x_max + 1]; vents.y_max + 1];

    for (c1, c2) in vents.lines {
        if c1.x == c2.x {
            let r = if c1.y < c2.y {
                c1.y..=c2.y
            } else {
                c2.y..=c1.y
            };

            for y in r {
                grid[y][c1.x] += 1;
            }
        }

        if c1.y == c2.y {
            let r = if c1.x < c2.x {
                c1.x..=c2.x
            } else {
                c2.x..=c1.x
            };

            for x in r {
                grid[c1.y][x] += 1;
            }
        }
    }

    grid.into_iter().flatten().filter(|v| *v >= 2).count()
}

fn part2(vents: Vents) -> usize {
    let mut grid: Vec<Vec<u16>> = vec![vec![0; vents.x_max + 1]; vents.y_max + 1];

    for (c1, c2) in vents.lines {
        // Vertical
        if c1.x == c2.x {
            let r = if c1.y < c2.y {
                c1.y..=c2.y
            } else {
                c2.y..=c1.y
            };

            for y in r {
                grid[y][c1.x] += 1;
            }
        }

        // Horizontal
        if c1.y == c2.y {
            let r = if c1.x < c2.x {
                c1.x..=c2.x
            } else {
                c2.x..=c1.x
            };

            for x in r {
                grid[c1.y][x] += 1;
            }
        }

        // Diagonal
        if (c1.x as isize - c2.x as isize).abs() == (c1.y as isize - c2.y as isize).abs() {
            let step = (c1.x as isize - c2.x as isize).abs();
            dbg!((c1, c2, step));
        }
    }

    grid.into_iter().flatten().filter(|v| *v >= 2).count()
}

#[test]
fn test_part1() {
    assert_eq!(5, part1(get_data("test.txt")));
    assert_eq!(5835, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(12, part2(get_data("test.txt")));
    // assert_eq!(19012, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vents {
    let data = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let mut vents = Vents::default();

    for line in data {
        let parts = line.split_once(" -> ").unwrap();
        let (x1, y1) = parts.0.split_once(',').unwrap();
        let (x2, y2) = parts.1.split_once(',').unwrap();

        let x1 = x1.parse().unwrap();
        let x2 = x2.parse().unwrap();
        let y1 = y1.parse().unwrap();
        let y2 = y2.parse().unwrap();

        if x1 > vents.x_max {
            vents.x_max = x1;
        }
        if x2 > vents.x_max {
            vents.x_max = x2;
        }
        if y1 > vents.y_max {
            vents.y_max = y1;
        }
        if y2 > vents.y_max {
            vents.y_max = y2;
        }

        let coordonnes1 = Coordinates { x: x1, y: y1 };
        let coordonnes2 = Coordinates { x: x2, y: y2 };

        vents.lines.push((coordonnes1, coordonnes2));
    }

    vents
}
