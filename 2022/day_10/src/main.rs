use std::{fs, result};

#[derive(Debug)]
enum Instruction {
    Noop, // 1 cycle
    Addx(isize), // 2 cycles
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Instruction>) -> usize {
    // cycles = 20 | 60 | 100 | 140 | 180 | 220
    // sum = v * 20 + k * 60 + j * 100 + i * 140 + m * 180 + q * 220
    dbg!(&data);
    let mut result = 0;
    let mut register = 1;
    let mut cycles = 0;

    for i in data {
        match i {
            Instruction::Noop => {

            },
            Instruction::Addx(v) => {

            },
        }
    }
    
    result
}

// fn part2(data: Vec<Instruction>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(13140, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Vec<Instruction> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| 
            if line == "noop" {
                Instruction::Noop
            } else {
                let (_, value) = line.split_once(' ').unwrap();
                Instruction::Addx(value.parse().unwrap())
            }
        )
        .collect()
}
