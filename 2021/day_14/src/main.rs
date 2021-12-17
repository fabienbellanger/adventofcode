use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt"), 10));
    println!("Part 2 result: {}", part2(get_data("input.txt"), 40));
}

fn part1(data: (Vec<char>, HashMap<(char, char), char>), steps: usize) -> usize {
    let mut polymer = data.0;
    let pairs = data.1;

    for _ in 0..steps {
        let n = polymer.len();
        let mut i = 0;
        loop {
            if i == 2 * n - 2 {
                break;
            }

            if let Some(c) = pairs.get(&(polymer[i], polymer[i + 1])) {
                polymer.insert(i + 1, *c);
            }

            i += 2;
        }
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in polymer {
        let e = counts.entry(c).or_insert(0);
        *e += 1;
    }

    let (mut min, mut max) = (usize::MAX, 0);
    for (_, n) in counts {
        if n > max {
            max = n
        };
        if n < min {
            min = n
        };
    }

    max - min
}

fn part2(data: (Vec<char>, HashMap<(char, char), char>), steps: usize) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(1588, part1(get_data("test.txt"), 10));
    assert_eq!(2937, part1(get_data("input.txt"), 10));
}

#[test]
fn test_part2() {
    assert_eq!(2188189693529, part2(get_data("test.txt"), 40));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let data: Vec<String> = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|n| n.to_string())
        .collect();

    let polymer = data.first().unwrap().chars().collect();

    let pairs = data
        .into_iter()
        .skip(2)
        .map(|line| {
            let (pattern, new) = line.split_once(" -> ").unwrap();
            let mut pattern = pattern.chars();
            let p1 = pattern.next().unwrap();
            let p2 = pattern.next().unwrap();

            let new = new.chars().next().unwrap();
            ((p1, p2), new)
        })
        .collect();

    (polymer, pairs)
}
