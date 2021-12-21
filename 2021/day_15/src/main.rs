use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
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
    let mut best = data.1;
    let data = data.0;

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut queue: VecDeque<((isize, isize), usize)> = VecDeque::new();
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

fn part2(
    data: (
        HashMap<(isize, isize), usize>,
        HashMap<(isize, isize), usize>,
        usize,
        usize,
    ),
) -> usize {
    let (mut y_max, mut x_max) = (data.2, data.3);
    let mut best = data.1;
    let data = data.0;

    let mut new_data: HashMap<(isize, isize), usize> = HashMap::new();
    let mut new_best: HashMap<(isize, isize), usize> = HashMap::new();

    const scale: isize = 5;

    for ((y, x), cost) in &data {
        for i in 0..scale {
            for j in 0..scale {
                // let y1 = y + j * scale;
                // let x1 = x + i * scale;
                let cost = *data.get(&(*y, *x)).unwrap() + 1;
                let cost = if cost > 9 { 1 } else { cost };
                new_data.insert((y + j * scale, x + i * scale), cost);
            }
        }
    }

    dbg!(new_data[&(4, 20)]);

    0
}

#[test]
fn test_part1() {
    assert_eq!(40, part1(get_data("test.txt")));
    assert_eq!(720, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(315, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
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
