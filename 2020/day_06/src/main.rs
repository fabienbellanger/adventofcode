use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(data: Vec<Vec<String>>) -> usize {
    let mut result = 0;
    
    for group in data {
        let mut good_answsers: HashMap<char, usize> = HashMap::new();
        for person in group {
            for c in person.chars() {
                good_answsers.insert(c, 1);
            }
        }
        result += good_answsers.len();
    }

    result
}

fn part2(data: Vec<Vec<String>>) -> usize {
    let mut result = 0;
    
    for group in data {
        let mut good_answsers: HashMap<char, usize> = HashMap::new();
        for person in &group {
            for c in person.chars() {
                let val = match good_answsers.get(&c) {
                    Some(v) => *v + 1,
                    None => 1,
                };
                good_answsers.insert(c, val);
            }
        }
        
        let mut good = 0;
        let persons_number = group.len();
        for (_, v) in &good_answsers {
            if *v == persons_number {
                good += 1;
            }
        }

        result += good;
    }

    result
}

fn get_groups(data: String) -> Vec<Vec<String>> {
    // Groups
    let data: Vec<&str> = data.trim().split("\n\n").collect();
    
    // Persons
    let mut groups: Vec<Vec<String>> = Vec::new();
    for group in data {
        let persons: Vec<String> = group.split("\n").map(|q| q.to_string()).collect();
        groups.push(persons);
    }

    groups
}

fn get_data() -> Vec<Vec<String>> {
    let data = fs::read_to_string("input.txt").expect("Cannot read the file input.txt");
    get_groups(data)
}

fn _get_data_test() -> Vec<Vec<String>> {
    let data = fs::read_to_string("test.txt").expect("Cannot read the file test.txt");
    get_groups(data)
}

#[test]
fn test_part1() {
    assert_eq!(11, part1(_get_data_test()));
    assert_eq!(6437, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(6, part2(_get_data_test()));
    assert_eq!(3229, part2(get_data()));
}
