use std::{fs, collections::HashMap};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: HashMap<(isize, isize), usize>) -> usize {
    dbg!(&data);
    let directions: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    0
}

// fn part2(data: HashMap<(isize, isize), usize>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(21, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> HashMap<(isize, isize), usize> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    ((x as isize, y as isize), c.to_digit(10).unwrap_or_default() as usize)
                })
                .collect::<HashMap<(isize, isize), usize>>()
        })
        .flatten()
        .collect()
}
