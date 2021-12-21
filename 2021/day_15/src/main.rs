use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn solve(
    data: HashMap<(isize, isize), usize>,
    best: HashMap<(isize, isize), usize>,
    y_max: usize,
    x_max: usize,
) -> usize {
    let mut best = best;
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    while let Some(((y, x), cost)) = queue.pop_front() {
        if cost < *best.get(&(y, x)).unwrap() {
            best.insert((y, x), cost);

            for (d_y, d_x) in &directions {
                let y = y + *d_y;
                let x = x + *d_x;

                if y >= 0 && y <= y_max as isize && x >= 0 && x <= x_max as isize {
                    queue.push_back(((y, x), cost + data.get(&(y, x)).unwrap()));
                }
            }
        }
    }

    *best.get(&(y_max as isize, x_max as isize)).unwrap_or(&0)
}

// See https://www.programiz.com/dsa/dijkstra-algorithm
fn part1(
    data: (
        HashMap<(isize, isize), usize>,
        HashMap<(isize, isize), usize>,
        usize,
        usize,
    ),
) -> usize {
    let (y_max, x_max) = (data.2, data.3);
    let best = data.1;
    let data = data.0;

    solve(data, best, y_max, x_max)
}

fn part2(
    data: (
        HashMap<(isize, isize), usize>,
        HashMap<(isize, isize), usize>,
        usize,
        usize,
    ),
) -> usize {
    const SCALE: isize = 5;

    let (y_max, x_max) = (data.2, data.3);
    let mut best: HashMap<(isize, isize), usize> = data.1;
    let data = data.0;

    let mut new_data: HashMap<(isize, isize), usize> = data.clone();

    for ((y, x), cost) in &data {
        for i in 0..SCALE {
            for j in 0..SCALE {
                if i == 0 && j == 0 {
                    continue;
                }

                let cost = get_cost(*cost + i as usize + j as usize);

                new_data.insert((y + i * (y_max as isize + 1), x + j * (x_max as isize + 1)), cost);
                best.insert((y + i * (y_max as isize + 1), x + j * (x_max as isize + 1)), usize::MAX);
            }
        }
    }

    let y_max = (y_max + 1) * SCALE as usize - 1;
    let x_max = (x_max + 1) * SCALE as usize - 1;

    solve(new_data, best, y_max, x_max)
}

fn get_cost(cost: usize) -> usize {
    if cost > 9 {
        cost % 9
    } else {
        cost
    }
}

fn _print_grid(data: &HashMap<(isize, isize), usize>, y_max: usize, x_max: usize, y_grid: usize, x_grid: usize) {
    for y in 0..=y_max {
        let mut s = String::new();
        for x in 0..=x_max {
            s.push_str(&format!("{} ", data.get(&(y as isize, x as isize)).unwrap_or(&0)));
            if x % (x_grid + 1) == x_grid {
                s.push_str(" ");
            }
        }
        println!("{}", s);
        if y % (y_grid + 1) == y_grid {
            println!();
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(40, part1(get_data("test.txt")));
    assert_eq!(720, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(315, part2(get_data("test.txt")));
    assert_eq!(3025, part2(get_data("input.txt")));
}

fn get_data(
    file: &str,
) -> (
    HashMap<(isize, isize), usize>,
    HashMap<(isize, isize), usize>,
    usize,
    usize,
) {
    let (mut y_max, mut x_max) = (0, 0);

    let data: HashMap<(isize, isize), usize> = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, v)| {
                    if row > y_max {
                        y_max = row;
                    }

                    if col > x_max {
                        x_max = col;
                    }

                    ((row as isize, col as isize), v.to_digit(10).unwrap() as usize)
                })
                .collect::<HashMap<(isize, isize), usize>>()
        })
        .flatten()
        .collect();

    let best: HashMap<(isize, isize), usize> = data.iter().map(|((row, col), _)| ((*row, *col), usize::MAX)).collect();

    (data, best, y_max, x_max)
}
