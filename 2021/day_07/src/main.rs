use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(positions: Vec<u16>) -> usize {
    let (mut min, mut max) = (u16::MAX, 0);
    for p in &positions {
        if min > *p {
            min = *p;
        }
        if max < *p {
            max = *p;
        }
    }

    (min..=max)
        .into_iter()
        .map(|i| {
            let s: usize = positions
                .iter()
                .map(|n| (*n as i16 - i as i16).abs() as usize)
                .sum();
            s
        })
        .min()
        .unwrap()
}

fn part2(positions: Vec<u16>) -> usize {
    let (mut min, mut max) = (u16::MAX, 0);
    for p in &positions {
        if min > *p {
            min = *p;
        }
        if max < *p {
            max = *p;
        }
    }

    (min..=max)
        .into_iter()
        .map(|i| {
            let s: usize = positions
                .iter()
                .map(|n| (*n as i16 - i as i16).abs() as usize)
                .map(|n| (1..=n).into_iter().sum::<usize>())
                .sum();
            s
        })
        .min()
        .unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(37, part1(get_data("test.txt")));
    assert_eq!(344297, part1(get_data("input.txt"))); // Too hight
}

#[test]
fn test_part2() {
    assert_eq!(168, part2(get_data("test.txt")));
    assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<u16> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
