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
    memories: HashMap<usize, Memory>,
}

impl Program {
    fn init() -> Self {
        Self {
            memories: HashMap::new(),
        }
    }
}

fn to_binary(memory: &mut Memory) {
    let bin = format!("{:b}", memory.val);
    
    for (i, v) in bin.chars().rev().enumerate() {
        memory.bin[i] = v.to_string().parse().unwrap();
    }
}

fn apply_mask(memory: &Memory, mask: &HashMap<usize, usize>) -> Memory {
    let mut new_value = memory.clone();

    println!("OLD={:?}", new_value.bin);
    for (index, _val) in memory.bin.iter().enumerate() {
        match mask.get(&index) {
            Some(v) => new_value.bin[index] = *v,
            None => (),
        };
    }
    println!("NEW={:?}\n", new_value.bin);

    // binary => usize
    let binary: String = new_value.bin.iter().rev().map(|l| l.to_string()).collect();
    let binary = isize::from_str_radix(&binary, 2).unwrap();
    new_value.val = binary as usize;

    new_value
}

fn main() {
    println!("Part 1 result: {}", part1(&get_data()));
    // println!("Part 2 result: {}", part2(&get_data()));
}

fn part1(program: &Program) -> usize {
    let mut sum = 0;

    for (_, mem_val) in &program.memories {
        sum += mem_val.val;
    }

    sum
}

fn _part2() -> isize {
    0
}

fn construct_programs(lines: Vec<String>) -> Program {
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let mut program: Program = Program::init();
    let mut mask = HashMap::new();

    for line in &lines {
        if line.contains("mask") {
            println!("------------------------------------");
            // Mask
            // ----
            let mask_str = &line[7..];
            println!("mask={:?}", mask_str);
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

            let mut mem = Memory::init();
            mem.val = value;
            to_binary(&mut mem);
            
            program.memories.insert(key, apply_mask(&mem, &mask));
        }
    }

    // dbg!(&program);

    program
}

fn get_data() -> Program {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect();

    construct_programs(lines)
}

fn _get_data_test() -> Program {
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
    assert_eq!(4886706177792, part1(&get_data()));
}

// #[test]
// fn test_part2() {
//     assert_eq!(, part2(&_get_data_test()));
//     assert_eq!(, part2(&get_data()));
// }
