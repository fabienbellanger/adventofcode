use std::{fs, usize};
use std::collections::HashMap;

const MAX_JOLTS: usize = 3;

fn main() {
    println!("Part 1 result: {}", part1(&mut get_data()));
    println!("Part 2 result: {}", part2(&mut get_data()));
}

fn part1(data: &mut [usize]) -> usize {
    data.sort_unstable();

    let mut diff_1 = 0;
    let mut diff_3 = 1;

    for (index, current) in data.iter().take(data.len() - 1).enumerate() {
        // Start with 0 value
        if index == 0 {
            match current {
                1 => diff_1 += 1,
                3 => diff_3 += 1,
                _ => (),
            }
        }

        let diff = data[index + 1] - current;
        match diff {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => (),
        }
    }

    diff_1 * diff_3
}

fn part2(data: &mut Vec<usize>) -> usize {
    data.push(0);
    data.sort_unstable();

    let mut values: HashMap<usize, usize> = HashMap::new();
    
    for (index, v) in data.iter().enumerate() {
        if index < 2 {
            values.insert(index, 1);
            continue;
        }

        let max = (*v).min(MAX_JOLTS);
        let mut val = 0;
        for i in 1..=max {
            val += values.get(&(v - i)).unwrap_or(&0);
        }
        
        values.insert(*v, val);
    }

    *values.get(data.last().unwrap()).unwrap()
}

fn get_data() -> Vec<usize> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn _get_data_test() -> Vec<usize> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[test]
fn test_part1() {
    let mut test = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    assert_eq!(35, part1(&mut test));
    assert_eq!(220, part1(&mut _get_data_test()));
    assert_eq!(2070, part1(&mut get_data()));
}

#[test]
fn test_part2() {
    let mut test = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    assert_eq!(8, part2(&mut test));
    assert_eq!(19208, part2(&mut _get_data_test()));
    assert_eq!(24179327893504, part2(&mut get_data()));
}
