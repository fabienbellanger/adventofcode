use std::{collections::VecDeque, fs};

#[derive(Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Debug)]
struct Test {
    value: usize,
    ok: usize,
    ko: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    test: Test,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Monkey>) -> usize {
    let mut inspected = vec![0; data.len()];
    let mut monkeys_items = data.iter().map(|m| m.items.clone()).collect::<Vec<VecDeque<usize>>>();

    for _ in 0..20 {
        for (j, monkey) in data.iter().enumerate() {
            for item in monkeys_items[j].clone() {
                let new = match monkey.op {
                    Operation::Add(v) => (item + v) / 3,
                    Operation::Mul(v) => (item * v) / 3,
                    Operation::Square => (item * item) / 3,
                };

                match new % monkey.test.value == 0 {
                    true => monkeys_items[monkey.test.ok].push_back(new),
                    false => monkeys_items[monkey.test.ko].push_back(new),
                }

                monkeys_items[j].pop_front();

                inspected[j] += 1;
            }
        }
    }

    inspected.sort_unstable();
    inspected.into_iter().rev().take(2).product::<usize>()
}

// fn part2(data: Vec<Monkey>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(10_605, part1(get_data("test.txt")));
    assert_eq!(57838, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Vec<Monkey> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .split("\n\n")
        .map(|s| {
            let mut lines = s.trim().lines().skip(1);

            // Items
            let (_, items_part) = lines.next().unwrap().split_once(": ").unwrap();
            let items = items_part.split(", ").map(|s| s.parse::<usize>().unwrap()).collect();

            // Operation
            let (_, items_op) = lines.next().unwrap().split_once("new = old ").unwrap();
            let op = match items_op.split_once(' ').unwrap() {
                ("+", v) => Operation::Add(v.parse::<usize>().unwrap()),
                ("*", "old") => Operation::Square,
                ("*", v) => Operation::Mul(v.parse::<usize>().unwrap()),
                _ => panic!(),
            };

            // Test
            let test_value = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let test_ok = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("If true: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let test_ko = lines
                .next()
                .unwrap()
                .trim()
                .strip_prefix("If false: throw to monkey ")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Monkey {
                items,
                op,
                test: Test {
                    value: test_value,
                    ok: test_ok,
                    ko: test_ko,
                },
            }
        })
        .collect::<Vec<Monkey>>()
}
