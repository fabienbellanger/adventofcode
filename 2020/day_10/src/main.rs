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
    data.sort_unstable();
    data.insert(0, 0);
    let n = data.len();

    let mut values: HashMap<usize, usize> = HashMap::new();
    values.insert(0, 1);
    values.insert(1, 1);
    
    for i in 2..n {
        let mut j = 1;
        let mut c: Vec<usize> = Vec::new();
        let max = if i == 2 {
            MAX_JOLTS
        } else {
            MAX_JOLTS + 1
        };
        while !(j == max + 1) {
            if i == 2 && data[i] == 2 {
                println!("{} + 1 - {} + 1 <= {}", data[i], MAX_JOLTS, data[i-j]);
                if data[i] >= max && data[i] + 1 - max <= data[i-j] {
                    println!("OK => {}, {}", data[i-j], data[i]);
                    c.push(data[i-j]);
                }
            } else {
                // println!("======> i = {}", i);
                if data[i] >= max && data[i] - max <= data[i-j] {
                    // println!("OK => {}, {}", data[i-j], data[i]);
                    c.push(data[i-j]);
                }
            }
            j += 1;
        }

        let mut t = 0;
        for k in c {
            t += values[&k];
        }
        values.insert(data[i], t);
    }
    
    dbg!(&values);

    values[&data[n-1]]
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
    //assert_eq!(35, part1(&mut _get_data_test()));
    assert_eq!(220, part1(&mut _get_data_test()));
    assert_eq!(2070, part1(&mut get_data()));
}

#[test]
fn test_part2() {
    // assert_eq!(8, part2(&mut _get_data_test()));
    assert_eq!(19208, part2(&mut _get_data_test()));
    // assert_eq!(, part2(&mut get_data()));
}
