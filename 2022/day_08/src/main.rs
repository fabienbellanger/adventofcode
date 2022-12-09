use std::{fs, collections::HashMap};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: (HashMap<(isize, isize), usize>, (usize, usize))) -> usize {
    let (x_max, y_max) = data.1;
    let data = data.0;

    let mut n = 0;
    for y in 1..y_max {
        for x in 1..x_max {
            let val = data[&(x as isize, y as isize)];
            
            // Top
            let top = (0..y)
                .map(|i| {
                    data[&(x as isize, i as isize)]
                })
                .all(|v| v < val);
            if top {
                n += 1;
                continue;
            }
            
            // Bottom
            let bottom = (y+1..=y_max)
                .map(|i| {
                    data[&(x as isize, i as isize)]
                })
                .all(|v| v < val);
            if bottom {
                n += 1;
                continue;
            }
            
            // Right
            let right = (x+1..=x_max)
                .map(|i| {
                    data[&(i as isize, y as isize)]
                })
                .all(|v| v < val);
            if right {
                n += 1;
                continue;
            }
            
            // Left
            let left = (0..x)
                .map(|i| {
                    data[&(i as isize, y as isize)]
                })
                .all(|v| v < val);
            if left {
                n += 1;
                continue;
            }
        }
    }

    2 * (x_max + y_max) + n
}

fn part2(data: (HashMap<(isize, isize), usize>, (usize, usize))) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(21, part1(get_data("test.txt")));
    assert_eq!(1827, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(8, part2(get_data("test.txt")));
    // assert_eq!(0, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (HashMap<(isize, isize), usize>, (usize, usize)) {
    let content = fs::read_to_string(file)
        .expect("Cannot read the file input.txt");

    let (mut y_max, mut x_max) = (0, 0);

    let data = content.trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if y > y_max {
                        y_max = y;
                    }

                    if x > x_max {
                        x_max = x;
                    }
                    ((x as isize, y as isize), c.to_digit(10).unwrap_or_default() as usize)
                })
                .collect::<HashMap<(isize, isize), usize>>()
        })
        .flatten()
        .collect();

    (data, (x_max, y_max))
}
