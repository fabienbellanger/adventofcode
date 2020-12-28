use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(lines: Vec<isize>) -> isize {
    // 2020 = x + y
    // x = 2020 - y
    // => x * y
    
    let mut m = HashMap::new();
    for line in &lines {
        m.insert(line, 0);
    }

    for y in &lines {
        let x: isize = 2020 - y;
        if m.get(&x).is_some() {
            dbg!(x, y);
            return x * y;
        }
    }

    panic!("No number found");
}

fn part2(lines: Vec<isize>) -> isize {
    // 2020 = x + y + z
    // 2020 - y = x + z
    // x + z = s
    // s - z = x
    // => x * y * z

    let mut m = HashMap::new();
    for line in &lines {
        m.insert(line, 0);
    }

    for y in &lines {
        let s: isize = 2020 - y;
        for x in &lines {
            let z = s - x;
            if m.get(&z).is_some() {
                return x * y * z;
            }
        }
    }

    panic!("No number found");
}

#[test]
fn test_part1() {
    assert_eq!(514579, part1(vec![1721, 979, 366, 299, 675, 1456]));
    assert_eq!(980499, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(241861950, part2(vec![1721, 979, 366, 299, 675, 1456]));
    assert_eq!(200637446, part2(get_data()));
}

fn get_data() -> Vec<isize> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
