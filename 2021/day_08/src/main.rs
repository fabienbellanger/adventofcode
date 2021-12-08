use std::fs;

#[derive(Debug, Default)]
struct Signal {
    signal_patterns: Vec<String>,
    ouput: Vec<String>,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(signals: Vec<Signal>) -> usize {
    signals
        .iter()
        .map(|s| {
            s.ouput
                .clone()
                .into_iter()
                .filter(|d| d.len() == 2 || d.len() == 3 || d.len() == 4 || d.len() == 7)
                .count()
        })
        .sum::<usize>()
}

fn part2(signals: Vec<Signal>) -> usize {
    let digits: [Vec<u8>; 10] = [
        vec![1, 2, 3, 5, 6, 7],    // 0
        vec![3, 6],                // 1
        vec![1, 3, 4, 5, 7],       // 2
        vec![1, 3, 4, 6, 7],       // 3
        vec![2, 3, 4, 6],          // 4
        vec![1, 2, 4, 6, 7],       // 5
        vec![1, 2, 4, 5, 6, 7],    // 6
        vec![1, 3, 6],             // 7
        vec![1, 2, 3, 4, 5, 6, 7], // 8
        vec![1, 2, 3, 4, 6, 7],    // 9
    ];

    for line in signals {
        let mut result: [u8; 7] = [0; 7];

        // 1. Find "1", with 1 et 7
        // 2. Find "7" with 2, 3, 5 (3x ("1", "4", "7"))
        // 3. Then "2" and "4"
        // 4. Find "5" with 0
        // 5. Find "3" and "6" with 6
    }

    0
}

fn group_by_length(list: Vec<String>, n: u8) -> Vec<String> {
    vec![]
}

#[test]
fn test_part1() {
    assert_eq!(26, part1(get_data("test.txt")));
    assert_eq!(247, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(61229, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Signal> {
    let mut signals = Vec::new();

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .for_each(|line| {
            let parts = line.split_once(" | ").unwrap();

            let signal_patterns: Vec<String> = parts
                .0
                .split_whitespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect();

            let ouput: Vec<String> = parts
                .1
                .split_whitespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect();

            signals.push(Signal {
                signal_patterns,
                ouput,
            });
        });

    signals
}
