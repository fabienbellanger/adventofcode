use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Instruction {
    operation: String,
    value: usize,
    is_positive: bool,
}

impl Instruction {
    fn new(operation: String, value: usize, is_positive: bool) -> Self {
        Self { operation, value, is_positive }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(data: Vec<Instruction>) -> usize {
    get_accumulator_1(data)
}

fn part2(instructions: Vec<Instruction>) -> usize {
    let mut result = 0;

    for (index, _instruction) in instructions.iter().enumerate() {
        let r = get_accumulator_2(&instructions, index);
        if let Some(o) = r {
            result = o;
        }
    }

    result
}

fn get_accumulator_1(instructions: Vec<Instruction>) -> usize {
    let mut used: HashSet<usize> = HashSet::new();
    let mut index: usize = 0;
    let mut accumulator = 0;

    loop {
        if used.contains(&index) {
            break;
        }

        used.insert(index);

        let instruction = instructions.get(index);
        match instruction {
            Some(instruction) => {
                match instruction.operation.as_str() {
                    "nop" => index += 1,
                    "acc" => {
                        if instruction.is_positive {
                            accumulator += instruction.value;
                        } else {
                            accumulator -= instruction.value;
                        }
                        index += 1;
                    }
                    "jmp" => {
                        if instruction.is_positive {
                            index += instruction.value;
                        } else {
                            index -= instruction.value;
                        }
                    },
                    _ => panic!("Operation not found"),
                };
            }
            None => panic!("Instruction not found"),
        };
    }

    accumulator
}

fn get_accumulator_2(data: &Vec<Instruction>, index: usize) -> Option<usize> {
    let mut instructions = data.clone();
    let mut used: HashSet<usize> = HashSet::new();
    let mut accumulator: isize = 0;

    for mut i in 0..instructions.len() {
        println!("BEFORE : acc: {}, op: {}, val: {}, pos: {}", accumulator, instructions[i].operation, instructions[i].value, instructions[i].is_positive);
        // Invertion NOP <=> JUMP
        if i == index && instructions[i].operation == "nop" {
            dbg!("NOP => JUMP");
            instructions[i].operation = "jmp".to_owned();
        }
        else if i == index && instructions[i].operation == "jmp" {
            dbg!("NOP <== JUMP");
            instructions[i].operation = "nop".to_owned();
        }
        if used.contains(&i) {
            return None;
        }

        used.insert(i);

        let instruction = instructions.get(i);
        match instruction {
            Some(instruction) => {
                println!("AFTER  : acc: {}, op: {}, val: {}, pos: {}", accumulator, instruction.operation, instruction.value, instruction.is_positive);
                match instruction.operation.as_str() {
                    "nop" => i += 1,
                    "acc" => {
                        let acc: isize = if instruction.is_positive {
                            instruction.value as isize
                        } else {
                            -(instruction.value as isize)
                        };
                        accumulator += acc;
                        i += 1;
                    }
                    "jmp" => {
                        if instruction.is_positive {
                            i += instruction.value;
                        } else {
                            i -= instruction.value;
                        }
                    },
                    _ => panic!("Operation not found"),
                };
            }
            None => panic!("Instruction not found"),
        };
    }
    dbg!(accumulator);
    Some(accumulator as usize)
}

fn get_instructions(data: Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for instruction in data {
        let mut parts = instruction.split_whitespace();
        let operation = parts.next().unwrap();
        let value_signed: isize = parts.next().unwrap().parse().unwrap();
        let is_positive = value_signed >= 0;
        let value: usize = if is_positive {
            value_signed as usize
        } else {
            -value_signed as usize
        };

        instructions.push(Instruction::new(operation.to_owned(), value, is_positive));
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
