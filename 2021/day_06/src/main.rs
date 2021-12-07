use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt"), 80));
    println!("Part 2 result: {}", part2(get_data("input.txt"), 256));
}

fn part1(initial_state: Vec<u8>, days: u16) -> usize {
    let mut state = initial_state;

    for _ in 0..days {
        let mut new_lanternfishs = 0;
        for n in state.iter_mut() {
            match n {
                0 => {
                    *n = 6;
                    new_lanternfishs += 1;
                }
                _ => *n -= 1,
            }
        }
        for _ in 0..new_lanternfishs {
            state.push(8);
        }
    }

    state.len()
}

fn part2(initial_state: Vec<u8>, days: u16) -> usize {
    let mut hash_state: HashMap<u8, usize> = HashMap::with_capacity(9);

    // Initialize hashmap
    for n in initial_state.into_iter() {
        let value = hash_state.entry(n).or_insert(0);
        *value += 1;
    }

    for _ in 0..days {
        let mut new_state = HashMap::with_capacity(9);

        for (timer, count) in &hash_state {
            if *timer == 0 {
                let t = new_state.entry(8).or_insert(0);
                *t += *count;
                let t = new_state.entry(6).or_insert(0);
                *t += *count;
            } else {
                let t = new_state.entry(*timer - 1).or_insert(0);
                *t += *count;
            }
        }

        hash_state = new_state;
    }

    hash_state.values().sum()
}

#[test]
fn test_part1() {
    assert_eq!(5934, part1(get_data("test.txt"), 80));
    assert_eq!(365862, part1(get_data("input.txt"), 80));
}

#[test]
fn test_part2() {
    assert_eq!(26984457539, part2(get_data("test.txt"), 256));
    assert_eq!(1653250886439, part2(get_data("input.txt"), 256));
}

fn get_data(file: &str) -> Vec<u8> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
