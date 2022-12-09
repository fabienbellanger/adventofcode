use std::{collections::HashMap, fs};

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
            let ranges = vec![
                (None, Some(0..y)),
                (None, Some(y + 1..y_max + 1)),
                (Some(x + 1..x_max + 1), None),
                (Some(0..x), None),
            ];

            for r in ranges {
                let all = match r {
                    (Some(r_x), _) => r_x.map(|i| data[&(i as isize, y as isize)]).all(|v| v < val),
                    (_, Some(r_y)) => r_y.map(|i| data[&(x as isize, i as isize)]).all(|v| v < val),
                    _ => break,
                };

                if all {
                    n += 1;
                    break;
                }
            }
        }
    }

    2 * (x_max + y_max) + n
}

fn part2(data: (HashMap<(isize, isize), usize>, (usize, usize))) -> usize {
    let (x_max, y_max) = data.1;
    let data = data.0;

    let mut max_score = 0;
    for y in 1..y_max {
        for x in 1..x_max {
            let val = data[&(x as isize, y as isize)];
            let ranges = vec![
                ('t', 0..y),
                ('b', y + 1..y_max + 1),
                ('r', x + 1..x_max + 1),
                ('l', 0..x),
            ];

            let mut score = 1usize;
            for (dir, r) in ranges {
                let mut dir_score = 0;
                for i in r {
                    let v = match dir {
                        't' => data[&(x as isize, (y - i - 1) as isize)],
                        'b' => data[&(x as isize, i as isize)],
                        'r' => data[&(i as isize, y as isize)],
                        'l' => data[&((x - i - 1) as isize, y as isize)],
                        _ => break,
                    };

                    dir_score += 1;
                    if val <= v {
                        break;
                    }
                }
                score *= dir_score;
            }
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

#[test]
fn test_part1() {
    assert_eq!(21, part1(get_data("test.txt")));
    assert_eq!(1_827, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(8, part2(get_data("test.txt")));
    assert_eq!(335_580, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (HashMap<(isize, isize), usize>, (usize, usize)) {
    let content = fs::read_to_string(file).expect("Cannot read the file input.txt");
    let (mut y_max, mut x_max) = (0, 0);

    let data = content
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
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
        .collect();

    (data, (x_max, y_max))
}
