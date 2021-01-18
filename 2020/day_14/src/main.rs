use std::{collections::HashMap, fs};
use regex::Regex;

#[derive(Debug)]
struct Memory {
    val: usize,
    bin: Vec<usize>,
}

#[derive(Debug)]
struct Program {
    mask: HashMap<usize, usize>,
    memories: HashMap<usize, Memory>,
}

impl Program {
    fn init() -> Self {
        Self {
            mask: HashMap::new(),
            memories: HashMap::new(),
        }
    }

    fn new_memory_value(&self, index: usize) -> usize {
        let mut new_value = self.memories.get(&index).unwrap().clone();
        let binary_val = to_binary(new_value);
        let mut new_binary: HashMap<usize, usize> = HashMap::new();

        for (index, val) in &binary_val {
            match self.mask.get(&index) {
                Some(v) => new_binary.insert(*index, *v),
                None => new_binary.insert(*index, *val),
            };
        }

        dbg!(&index, &binary_val, &new_binary);
        
        0
    }
}

fn to_binary(val: usize) -> HashMap<usize, usize> {
    let b = format!("{:b}", val);
    let mut hm = HashMap::new();
    for (i, v) in b.chars().rev().enumerate() {
        hm.insert(i, v.to_string().parse().unwrap());
    }

    hm
}

fn main() {
    println!("Part 1 result: {}", part1(&get_data()));
    // println!("Part 2 result: {}", part2(&get_data()));
}

fn part1(programs: &Vec<Program>) -> usize {
    dbg!(&programs);

    let mut sum = 0;

    for program in programs {
        for (mem_index, mem_val) in &program.memories {
            program.new_memory_value(*mem_index);
        }
    }

    sum
}

fn _part2() -> isize {
    0
}

fn construct_programs(lines: Vec<String>) -> Vec<Program> {
    let mut programs: Vec<Program> = Vec::new();
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut program: Program = Program::init();
    for line in &lines {
        if line.contains("mask") {
            // Mask
            // ----
            if program.memories.len() > 0 {
                programs.push(program);
                program = Program::init();
            }

            let mask = &line[7..];
            for (index, bit) in mask.chars().rev().enumerate() {
                match bit {
                    '0' => program.mask.insert(index, 0),
                    '1' => program.mask.insert(index, 1),
                    _ => None,
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
            program.memories.insert(key, value);
        }
    }
    programs.push(program);

    programs
}

fn get_data() -> Vec<Program> {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    construct_programs(lines)
}

fn _get_data_test() -> Vec<Program> {
    let lines: Vec<String> = fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    construct_programs(lines)
}

#[test]
fn test_part1() {
    assert_eq!(165, part1(&_get_data_test()));
    // assert_eq!(, part1(&get_data()));
}

// #[test]
// fn test_part2() {
//     assert_eq!(, part2(&_get_data_test()));
//     assert_eq!(, part2(&get_data()));
// }
