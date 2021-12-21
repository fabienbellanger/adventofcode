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
    const SCALE: isize = 5;

    let (y_max, x_max) = (data.2, data.3);
    let data = data.0;
    print_grid(&data, y_max, x_max, y_max, x_max);

    let mut new_data: HashMap<(isize, isize), usize> = data.clone();
    let mut _best: HashMap<(isize, isize), usize> = HashMap::new();

    // 1 1 6 3 7 5 1 7 4 2  2 2 7 4 8 6 2 8 5 3  3 3 8 5 9 7 3 9 6 4  4 4 9 6 1 8 4 1 7 5  5 5 1 7 2 9 5 2 8 6
    // 1 3 8 1 3 7 3 6 7 2  2 4 9 2 4 8 4 7 8 3  3 5 1 3 5 9 5 8 9 4  4 6 2 4 6 1 6 9 1 5  5 7 3 5 7 2 7 1 2 6
    // 2 1 3 6 5 1 1 3 2 8  3 2 4 7 6 2 2 4 3 9  4 3 5 8 7 3 3 5 4 1  5 4 6 9 8 4 4 6 5 2  6 5 7 1 9 5 5 7 6 3
    // 3 6 9 4 9 3 1 5 6 9  4 7 1 5 1 4 2 6 7 1  5 8 2 6 2 5 3 7 8 2  6 9 3 7 3 6 4 8 9 3  7 1 4 8 4 7 5 9 1 4
    // 7 4 6 3 4 1 7 1 1 1  8 5 7 4 5 2 8 2 2 2  9 6 8 5 6 3 9 3 3 3  1 7 9 6 7 4 1 4 4 4  2 8 1 7 8 5 2 5 5 5
    // 1 3 1 9 1 2 8 1 3 7  2 4 2 1 2 3 9 2 4 8  3 5 3 2 3 4 1 3 5 9  4 6 4 3 4 5 2 4 6 1  5 7 5 4 5 6 3 5 7 2
    // 1 3 5 9 9 1 2 4 2 1  2 4 6 1 1 2 3 5 3 2  3 5 7 2 2 3 4 6 4 3  4 6 8 3 3 4 5 7 5 4  5 7 9 4 4 5 6 8 6 5
    // 3 1 2 5 4 2 1 6 3 9  4 2 3 6 5 3 2 7 4 1  5 3 4 7 6 4 3 8 5 2  6 4 5 8 7 5 4 9 6 3  7 5 6 9 8 6 5 1 7 4
    // 1 2 9 3 1 3 8 5 2 1  2 3 1 4 2 4 9 6 3 2  3 4 2 5 3 5 1 7 4 3  4 5 3 6 4 6 2 8 5 4  5 6 4 7 5 7 3 9 6 5
    // 2 3 1 1 9 4 4 5 8 1  3 4 2 2 1 5 5 6 9 2  4 5 3 3 2 6 6 7 1 3  5 6 4 4 3 7 7 8 2 4  6 7 5 5 4 8 8 9 3 5
    //
    // 2 2 7 4 8 6 2 8 5 3  3 3 8 5 9 7 3 9 6 4  4 4 9 6 1 8 4 1 7 5  5 5 1 7 2 9 5 2 8 6  6 6 2 8 3 1 6 3 9 7
    // 2 4 9 2 4 8 4 7 8 3  3 5 1 3 5 9 5 8 9 4  4 6 2 4 6 1 6 9 1 5  5 7 3 5 7 2 7 1 2 6  6 8 4 6 8 3 8 2 3 7
    // 3 2 4 7 6 2 2 4 3 9  4 3 5 8 7 3 3 5 4 1  5 4 6 9 8 4 4 6 5 2  6 5 7 1 9 5 5 7 6 3  7 6 8 2 1 6 6 8 7 4
    // 4 7 1 5 1 4 2 6 7 1  5 8 2 6 2 5 3 7 8 2  6 9 3 7 3 6 4 8 9 3  7 1 4 8 4 7 5 9 1 4  8 2 5 9 5 8 6 1 2 5
    // 8 5 7 4 5 2 8 2 2 2  9 6 8 5 6 3 9 3 3 3  1 7 9 6 7 4 1 4 4 4  2 8 1 7 8 5 2 5 5 5  3 9 2 8 9 6 3 6 6 6
    // 2 4 2 1 2 3 9 2 4 8  3 5 3 2 3 4 1 3 5 9  4 6 4 3 4 5 2 4 6 1  5 7 5 4 5 6 3 5 7 2  6 8 6 5 6 7 4 6 8 3
    // 2 4 6 1 1 2 3 5 3 2  3 5 7 2 2 3 4 6 4 3  4 6 8 3 3 4 5 7 5 4  5 7 9 4 4 5 6 8 6 5  6 8 1 5 5 6 7 9 7 6
    // 4 2 3 6 5 3 2 7 4 1  5 3 4 7 6 4 3 8 5 2  6 4 5 8 7 5 4 9 6 3  7 5 6 9 8 6 5 1 7 4  8 6 7 1 9 7 6 2 8 5
    // 2 3 1 4 2 4 9 6 3 2  3 4 2 5 3 5 1 7 4 3  4 5 3 6 4 6 2 8 5 4  5 6 4 7 5 7 3 9 6 5  6 7 5 8 6 8 4 1 7 6
    // 3 4 2 2 1 5 5 6 9 2  4 5 3 3 2 6 6 7 1 3  5 6 4 4 3 7 7 8 2 4  6 7 5 5 4 8 8 9 3 5  7 8 6 6 5 9 9 1 4 6
    //
    // 3 3 8 5 9 7 3 9 6 4  4 4 9 6 1 8 4 1 7 5  5 5 1 7 2 9 5 2 8 6  6 6 2 8 3 1 6 3 9 7  7 7 3 9 4 2 7 4 1 8
    // 3 5 1 3 5 9 5 8 9 4  4 6 2 4 6 1 6 9 1 5  5 7 3 5 7 2 7 1 2 6  6 8 4 6 8 3 8 2 3 7  7 9 5 7 9 4 9 3 4 8
    // 4 3 5 8 7 3 3 5 4 1  5 4 6 9 8 4 4 6 5 2  6 5 7 1 9 5 5 7 6 3  7 6 8 2 1 6 6 8 7 4  8 7 9 3 2 7 7 9 8 5
    // 5 8 2 6 2 5 3 7 8 2  6 9 3 7 3 6 4 8 9 3  7 1 4 8 4 7 5 9 1 4  8 2 5 9 5 8 6 1 2 5  9 3 6 1 6 9 7 2 3 6
    // 9 6 8 5 6 3 9 3 3 3  1 7 9 6 7 4 1 4 4 4  2 8 1 7 8 5 2 5 5 5  3 9 2 8 9 6 3 6 6 6  4 1 3 9 1 7 4 7 7 7
    // 3 5 3 2 3 4 1 3 5 9  4 6 4 3 4 5 2 4 6 1  5 7 5 4 5 6 3 5 7 2  6 8 6 5 6 7 4 6 8 3  7 9 7 6 7 8 5 7 9 4
    // 3 5 7 2 2 3 4 6 4 3  4 6 8 3 3 4 5 7 5 4  5 7 9 4 4 5 6 8 6 5  6 8 1 5 5 6 7 9 7 6  7 9 2 6 6 7 8 1 8 7
    // 5 3 4 7 6 4 3 8 5 2  6 4 5 8 7 5 4 9 6 3  7 5 6 9 8 6 5 1 7 4  8 6 7 1 9 7 6 2 8 5  9 7 8 2 1 8 7 3 9 6
    // 3 4 2 5 3 5 1 7 4 3  4 5 3 6 4 6 2 8 5 4  5 6 4 7 5 7 3 9 6 5  6 7 5 8 6 8 4 1 7 6  7 8 6 9 7 9 5 2 8 7
    // 4 5 3 3 2 6 6 7 1 3  5 6 4 4 3 7 7 8 2 4  6 7 5 5 4 8 8 9 3 5  7 8 6 6 5 9 9 1 4 6  8 9 7 7 6 1 1 2 5 7
    //
    // 4 4 9 6 1 8 4 1 7 5  5 5 1 7 2 9 5 2 8 6  6 6 2 8 3 1 6 3 9 7  7 7 3 9 4 2 7 4 1 8  8 8 4 1 5 3 8 5 2 9
    // 4 6 2 4 6 1 6 9 1 5  5 7 3 5 7 2 7 1 2 6  6 8 4 6 8 3 8 2 3 7  7 9 5 7 9 4 9 3 4 8  8 1 6 8 1 5 1 4 5 9
    // 5 4 6 9 8 4 4 6 5 2  6 5 7 1 9 5 5 7 6 3  7 6 8 2 1 6 6 8 7 4  8 7 9 3 2 7 7 9 8 5  9 8 1 4 3 8 8 1 9 6
    // 6 9 3 7 3 6 4 8 9 3  7 1 4 8 4 7 5 9 1 4  8 2 5 9 5 8 6 1 2 5  9 3 6 1 6 9 7 2 3 6  1 4 7 2 7 1 8 3 4 7
    // 1 7 9 6 7 4 1 4 4 4  2 8 1 7 8 5 2 5 5 5  3 9 2 8 9 6 3 6 6 6  4 1 3 9 1 7 4 7 7 7  5 2 4 1 2 8 5 8 8 8
    // 4 6 4 3 4 5 2 4 6 1  5 7 5 4 5 6 3 5 7 2  6 8 6 5 6 7 4 6 8 3  7 9 7 6 7 8 5 7 9 4  8 1 8 7 8 9 6 8 1 5
    // 4 6 8 3 3 4 5 7 5 4  5 7 9 4 4 5 6 8 6 5  6 8 1 5 5 6 7 9 7 6  7 9 2 6 6 7 8 1 8 7  8 1 3 7 7 8 9 2 9 8
    // 6 4 5 8 7 5 4 9 6 3  7 5 6 9 8 6 5 1 7 4  8 6 7 1 9 7 6 2 8 5  9 7 8 2 1 8 7 3 9 6  1 8 9 3 2 9 8 4 1 7
    // 4 5 3 6 4 6 2 8 5 4  5 6 4 7 5 7 3 9 6 5  6 7 5 8 6 8 4 1 7 6  7 8 6 9 7 9 5 2 8 7  8 9 7 1 8 1 6 3 9 8
    // 5 6 4 4 3 7 7 8 2 4  6 7 5 5 4 8 8 9 3 5  7 8 6 6 5 9 9 1 4 6  8 9 7 7 6 1 1 2 5 7  9 1 8 8 7 2 2 3 6 8
    //
    // 5 5 1 7 2 9 5 2 8 6  6 6 2 8 3 1 6 3 9 7  7 7 3 9 4 2 7 4 1 8  8 8 4 1 5 3 8 5 2 9  9 9 5 2 6 4 9 6 3 1
    // 5 7 3 5 7 2 7 1 2 6  6 8 4 6 8 3 8 2 3 7  7 9 5 7 9 4 9 3 4 8  8 1 6 8 1 5 1 4 5 9  9 2 7 9 2 6 2 5 6 1
    // 6 5 7 1 9 5 5 7 6 3  7 6 8 2 1 6 6 8 7 4  8 7 9 3 2 7 7 9 8 5  9 8 1 4 3 8 8 1 9 6  1 9 2 5 4 9 9 2 1 7
    // 7 1 4 8 4 7 5 9 1 4  8 2 5 9 5 8 6 1 2 5  9 3 6 1 6 9 7 2 3 6  1 4 7 2 7 1 8 3 4 7  2 5 8 3 8 2 9 4 5 8
    // 2 8 1 7 8 5 2 5 5 5  3 9 2 8 9 6 3 6 6 6  4 1 3 9 1 7 4 7 7 7  5 2 4 1 2 8 5 8 8 8  6 3 5 2 3 9 6 9 9 9
    // 5 7 5 4 5 6 3 5 7 2  6 8 6 5 6 7 4 6 8 3  7 9 7 6 7 8 5 7 9 4  8 1 8 7 8 9 6 8 1 5  9 2 9 8 9 1 7 9 2 6
    // 5 7 9 4 4 5 6 8 6 5  6 8 1 5 5 6 7 9 7 6  7 9 2 6 6 7 8 1 8 7  8 1 3 7 7 8 9 2 9 8  9 2 4 8 8 9 1 3 1 9
    // 7 5 6 9 8 6 5 1 7 4  8 6 7 1 9 7 6 2 8 5  9 7 8 2 1 8 7 3 9 6  1 8 9 3 2 9 8 4 1 7  2 9 1 4 3 1 9 5 2 8
    // 5 6 4 7 5 7 3 9 6 5  6 7 5 8 6 8 4 1 7 6  7 8 6 9 7 9 5 2 8 7  8 9 7 1 8 1 6 3 9 8  9 1 8 2 9 2 7 4 1 9
    // 6 7 5 5 4 8 8 9 3 5  7 8 6 6 5 9 9 1 4 6  8 9 7 7 6 1 1 2 5 7  9 1 8 8 7 2 2 3 6 8  1 2 9 9 8 3 3 4 7 9

    for ((y, x), cost) in &data {
        for i in 0..SCALE {
            for j in 0..SCALE {
                if i == 0 && j == 0 {
                    continue;
                }

                let cost = *data.get(&(*y, *x)).unwrap() + i as usize + j as usize;
                let cost = get_cost(cost);
                new_data.insert((y + i * (y_max as isize + 1), x + j * (x_max as isize + 1)), cost);
            }
        }
    }

    println!();
    print_grid(
        &new_data,
        (y_max + 1) * SCALE as usize - 1,
        (x_max + 1) * SCALE as usize - 1,
        y_max,
        x_max,
    );

    0
}

fn get_cost(cost: usize) -> usize {
    if cost > 9 {
        cost % 9
    } else {
        cost
    }
}

fn print_grid(data: &HashMap<(isize, isize), usize>, y_max: usize, x_max: usize, y_grid: usize, x_grid: usize) {
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
