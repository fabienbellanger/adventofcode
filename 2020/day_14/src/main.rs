use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Program {
    mask: HashMap<usize, usize>,
    memories: HashMap<usize, usize>,
}

fn main() {
    dbg!(_get_data_test());
    // println!("Part 1 result: {}", part1(&get_data()));
    // println!("Part 2 result: {}", part2(&get_data()));
}

fn part1() -> isize {
    0
}

fn part2() -> isize {
    0
}

// fn get_data() -> Vec<Instruction> {
//     fs::read_to_string("input.txt")
//         .expect("Cannot read the file input.txt")
//         .trim()
//         .lines()
//         .map(|line| {
//             let (action, value) = line.trim().split_at(1);

//             Instruction::new(action, value)
//         })
//         .collect()
// }

fn _get_data_test() {
    let lines: Vec<String> = fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect();
    
    dbg!(&lines);
    for line in lines {
        
    }
}

#[test]
fn test_part1() {
    _get_data_test();
    // assert_eq!(25, part1(&_get_data_test()));
    // assert_eq!(759, part1(&get_data()));
}

// #[test]
// fn test_part2() {
//     assert_eq!(286, part2(&_get_data_test()));
//     assert_eq!(45763, part2(&get_data()));
// }

