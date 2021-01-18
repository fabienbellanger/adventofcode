use std::{collections::HashMap, fs};
use regex::Regex;

#[derive(Debug, Clone)]
struct Memory {
    val: usize,
    bin: Vec<usize>,
}

impl Memory {
    fn init() -> Self {
        Self {
            val: 0,
            bin: vec![0;36],
        }
    }
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

    fn new_memory_value(&self, index: usize) -> Memory {
        let binary = self.memories.get(&index).unwrap();
        let mut new_value = binary.clone();

        for (index, _val) in binary.bin.iter().enumerate() {
            match self.mask.get(&index) {
                Some(v) => new_value.bin[index] = *v,
                None => (),
            };
        }

        // binary => usize
        let binary: String = new_value.bin.iter().rev().map(|l| l.to_string()).collect();
        let binary = isize::from_str_radix(&binary, 2).unwrap();
        new_value.val = binary as usize;
        
        new_value
    }
}

fn to_binary(memory: &mut Memory) {
    let bin = format!("{:b}", memory.val);
    
    for (i, v) in bin.chars().rev().enumerate() {
        memory.bin[i] = v.to_string().parse().unwrap();
    }
}

fn main() {
    println!("Part 1 result: {}", part1(&get_data()));
    // println!("Part 2 result: {}", part2(&get_data()));
}

fn part1(programs: &Vec<Program>) -> usize {
    let mut sum = 0;

    for program in programs {
        for (mem_index, _) in &program.memories {
            let new_memory = program.new_memory_value(*mem_index);

            sum += new_memory.val;
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

            let mut mem = Memory::init();
            mem.val = value;
            to_binary(&mut mem);
            program.memories.insert(key, mem);
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
