use std::{collections::HashMap, fs};
use regex::Regex;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(program: HashMap<usize, usize>) -> usize {
    program.values().sum()
}

fn part2(program: HashMap<usize, usize>) -> isize {
    0
}

fn get_data() -> HashMap<usize, usize> {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    construct_program(lines)
}

fn _get_data_test() -> HashMap<usize, usize> {
    let lines: Vec<String> = fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    construct_program(lines)
}

fn _get_data_test_2() -> HashMap<usize, usize> {
    let lines: Vec<String> = fs::read_to_string("test2.txt")
        .expect("Cannot read the file test2.txt")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    construct_program(lines)
}

// GOOD
fn number_to_binary(value: usize) -> Vec<usize> {
    let binary = vec![0;36];
    let mut new_binary = binary.clone();
    let val_to_bin: Vec<usize> = format!("{:b}", value)
        .chars()
        .map(|c| {
            c.to_string().parse().unwrap()
        })
        .rev()
        .collect();
    
    for (i, _) in binary.iter().enumerate() {
        if let Some(bit) = val_to_bin.get(i) {
            new_binary[i] = *bit;
        }
    }
    
    new_binary
}

// GOOD
fn binary_to_number(binary: Vec<usize>) -> usize {
    let mut value = 0;

    for (i, b) in binary.iter().enumerate() {
        value += *b * 2usize.pow(i as u32);
    }

    value
}

fn apply_mask(binaray: Vec<usize>, mask: &HashMap<usize, usize>) -> Vec<usize> {
    let mut new_binary = Vec::new();

    for (i, bit) in binaray.iter().enumerate() {
        match mask.get(&i) {
            Some(b) => new_binary.push(*b),
            None => new_binary.push(*bit),
        }
    }

    new_binary
}

fn construct_program(lines: Vec<String>) -> HashMap<usize, usize> {
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let mut mask: HashMap<usize, usize> = HashMap::new();
    let mut addresses: HashMap<usize, usize> = HashMap::new();

    for line in &lines {
        if line.contains("mask") {
            // Mask
            // ----
            mask = HashMap::new();

            let mask_str = &line[7..];
            for (index, bit) in mask_str.chars().rev().enumerate() {
                match bit {
                    c if c == '0' || c == '1' => &mask.insert(index, c.to_string().parse().unwrap()),
                    _ => &None,
                };
            }
        } else {
            // Memory
            // ------
            if !re.is_match(line) {
                continue;
            }
    
            let cap = re.captures(line).unwrap();
            if cap.len() != 3 {
                continue;
            }
            let key: usize = (&cap[1]).parse().unwrap();
            let value: usize = (&cap[2]).parse().unwrap();
            let value = binary_to_number(apply_mask(number_to_binary(value), &mask));

            addresses.insert(key, value);
        }
    }

    addresses
}

fn _combine(add: Vec<usize>, perm: &Vec<usize>, index: usize) -> Vec<Vec<usize>> {
    if index == perm.len() {
        return vec![add];
    }
    
    let mut add1 = add.clone();
    let mut add2 = add.clone();
    add1[perm[index]] = 0;
    add2[perm[index]] = 1;
    
    let mut p1 = combine(add1, &perm, index+1);
    let mut p2 = combine(add2, &perm, index+1);
    // dbg!(&perm, &p1, &p2);
    p1.append(&mut p2);
    
    p1
}

#[test]
fn test_binary_to_number() {
    assert_eq!(125580, binary_to_number(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
}

#[test]
fn test_number_to_binary() {
    assert_eq!(vec![0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], number_to_binary(125580));
}

#[test]
fn test_part1() {
    assert_eq!(165, part1(_get_data_test()));
    assert_eq!(4886706177792, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(208, part2(_get_data_test_2()));
    // assert_eq!(, part2(get_data()));
}
