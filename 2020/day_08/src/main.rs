use std::fs;

#[derive(Debug)]
struct Instruction {
    operation: String,
    value: isize,
}

impl Instruction {
    fn new(operation: String, value: isize) -> Self {
        Self { operation, value }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    // println!("Part 2 result: {}", part2(get_data()));
}

fn part1(data: Vec<Instruction>) -> usize {
    let mut result = 0;

    dbg!(data);

    result
}

fn get_instructions(data: Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for instruction in data {
        let mut parts = instruction.split_whitespace();
        let operation = parts.next().unwrap();
        let value: isize = parts.next().unwrap().parse().unwrap();

        instructions.push(Instruction::new(operation.to_owned(), value));
    }

    instructions
}

fn get_data() -> Vec<Instruction> {
    let data: Vec<String> = fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect();

    get_instructions(data)
}

fn _get_data_test() -> Vec<Instruction> {
    let data = fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect();

    get_instructions(data)
}

#[test]
fn test_part1() {
    assert_eq!(4, part1(_get_data_test()));
    assert_eq!(179, part1(get_data()));
}
