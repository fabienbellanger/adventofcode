use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
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
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(data: Vec<Instruction>) -> isize {
    get_accumulator_1(data)
}

fn part2(data: Vec<Instruction>) -> isize {
    let mut result = 0;

    for (index, _instruction) in data.iter().enumerate() {
        let r = run_program(&data, index as isize);
        if r != 0 {
            result = r;
        }
    }

    result
}

fn run_program(instructions: &Vec<Instruction>, flip: isize) -> isize {
    let mut i: isize = 0;
    let mut visited: HashSet<isize> = HashSet::new();
    let mut acc = 0isize;

    while i < instructions.len() as isize {
        println!("i={}, flip={}", i as usize, flip);

        if visited.contains(&i) {
            break;
        }

        visited.insert(i);

        let mut op = instructions.get(i as usize).unwrap().operation.as_str();
        if i == flip {
            if op == "jmp" {
                op = "nop";
            }
            if op == "nop" {
                op = "jmp";
            }
        }

        match op {
            "nop" => i += 1,
            "acc" => {
                acc += instructions[i as usize].value;
                i += 1;
            }
            "jmp" => i += instructions[i as usize].value,
            _ => panic!("Operation not found"),
        }

        dbg!(acc);
    }

    acc
}

fn get_accumulator_1(instructions: Vec<Instruction>) -> isize {
    let mut used: HashSet<isize> = HashSet::new();
    let mut index: isize = 0;
    let mut accumulator = 0;

    loop {
        if used.contains(&index) {
            break;
        }

        used.insert(index);

        let instruction = instructions.get(index as usize);
        match instruction {
            Some(instruction) => {
                match instruction.operation.as_str() {
                    "nop" => index += 1,
                    "acc" => {
                        accumulator += instruction.value;
                        index += 1;
                    }
                    "jmp" => index += instruction.value,
                    _ => panic!("Operation not found"),
                };
            }
            None => panic!("Instruction not found"),
        };
    }

    accumulator
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
    assert_eq!(5, part1(_get_data_test()));
    assert_eq!(1487, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(8, part2(_get_data_test()));
    // assert_eq!(1487, part2(get_data()));
}
