use std::fs;

const CYCLES: [isize; 6] = [19, 59, 99, 139, 179, 219];

#[derive(Debug)]
enum Instruction {
    Noop,        // 1 cycle
    Addx(isize), // 2 cycles
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Instruction>) -> isize {
    let mut result = 0;
    let mut register = 1;
    let mut cycles = 0;

    for i in data {
        cycles += 1;
        if CYCLES.contains(&cycles) {
            result += (cycles + 1) * register;
        }

        if let Instruction::Addx(v) = i {
            cycles += 1;
            register += v;
            if CYCLES.contains(&cycles) {
                result += (cycles + 1) * register;
            }
        }
    }

    result
}

fn part2(data: Vec<Instruction>) -> isize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(13140, part1(get_data("test.txt")));
    assert_eq!(15360, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(0, part2(get_data("test.txt")));
    // assert_eq!(0, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Instruction> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| {
            if line == "noop" {
                Instruction::Noop
            } else {
                let (_, value) = line.split_once(' ').unwrap();
                Instruction::Addx(value.parse().unwrap())
            }
        })
        .collect()
}
