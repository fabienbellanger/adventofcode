use std::fs;

struct Crab {
    positions: Vec<u16>,
    min: u16,
    max: u16,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(crab: Crab) -> usize {
    (crab.min..=crab.max)
        .into_iter()
        .map(|i| {
            crab.positions
                .iter()
                .map(|n| (*n as i16 - i as i16).abs() as usize)
                .sum::<usize>()
        })
        .min()
        .unwrap_or_default()
}

fn part2(crab: Crab) -> usize {
    (crab.min..=crab.max)
        .into_iter()
        .map(|i| {
            crab.positions
                .iter()
                .map(|n| (*n as i16 - i as i16).abs() as usize)
                .map(|n| (1..=n).into_iter().sum::<usize>())
                .sum::<usize>()
        })
        .min()
        .unwrap_or_default()
}

#[test]
fn test_part1() {
    assert_eq!(37, part1(get_data("test.txt")));
    assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(168, part2(get_data("test.txt")));
    assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Crab {
    let mut crab = Crab {
        positions: vec![],
        min: u16::MAX,
        max: 0,
    };

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .split(',')
        .for_each(|n| {
            let n = n.parse().unwrap();

            if crab.min > n {
                crab.min = n;
            }

            if crab.max < n {
                crab.max = n;
            }

            crab.positions.push(n);
        });

    crab
}
