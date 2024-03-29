use std::{collections::HashMap, fs};

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
    let mut sum = 0;
    for line in signals {
        let result = find_segments(&line.signal_patterns);

        sum += get_output_value(&line.ouput, result);
    }

    sum
}

fn get_signal_by_length(line: &[String]) -> HashMap<usize, Vec<String>> {
    let mut result: HashMap<usize, Vec<String>> = HashMap::with_capacity(6);

    line.iter().for_each(|l| {
        let n = l.chars().count();
        let item = result.entry(n).or_insert_with(Vec::new);
        item.push(l.clone());
    });

    result
}

fn find_segments(line: &[String]) -> [char; 7] {
    let signals_by_length = get_signal_by_length(line);
    let mut result: [char; 7] = [' '; 7];

    // Step 1 ('0' => in 7 but not in 1)
    // ---------------------------------
    let find_1: Vec<_> = signals_by_length
        .get(&3)
        .unwrap()
        .first()
        .unwrap()
        .chars()
        .filter(|c| !signals_by_length.get(&2).unwrap().first().unwrap().contains(*c))
        .collect();
    result[0] = *find_1.first().unwrap();

    // Step 2 ('6', '3', '1' => in 2, 3, 5 - in (7 or 1 & 4)
    // ------------------------------------------------
    let mut step_2: HashMap<char, u8> = HashMap::with_capacity(5);
    signals_by_length.get(&5).unwrap().iter().for_each(|s| {
        s.chars().for_each(|c| {
            let e = step_2.entry(c).or_insert(0);
            *e += 1;
        })
    });
    let step_2 = step_2
        .into_iter()
        .filter(|(c, n)| *n == 3 && *c != result[0])
        .map(|v| v.0)
        .collect::<Vec<char>>();

    for c in signals_by_length.get(&4).unwrap().first().unwrap().chars() {
        if step_2.contains(&c) {
            result[3] = c;
            break;
        }
    }
    result[6] = step_2.into_iter().find(|c| *c != result[3]).unwrap();
    result[1] = signals_by_length
        .get(&4)
        .unwrap()
        .first()
        .unwrap()
        .chars()
        .find(|c| *c != result[3] && !signals_by_length.get(&2).unwrap().first().unwrap().contains(*c))
        .unwrap();

    // Step 3 ('4' => all in 6 digitis which are the same and not in result)
    // ---------------------------------------------------------------------
    for s in signals_by_length.get(&6).unwrap() {
        for c in s.chars() {
            if !result.contains(&c) && !signals_by_length.get(&2).unwrap().first().unwrap().contains(c) {
                result[4] = c;
            }
        }
    }

    // Step 4
    // ------
    let mut step_4: HashMap<char, u8> = HashMap::new();
    for s in signals_by_length.get(&2).unwrap() {
        for c in s.chars() {
            if !result.contains(&c) {
                for s1 in signals_by_length.get(&5).unwrap() {
                    for c1 in s1.chars() {
                        if c == c1 {
                            let e = step_4.entry(c1).or_insert(0);
                            *e += 1;
                        }
                    }
                }
                for s1 in signals_by_length.get(&6).unwrap() {
                    for c1 in s1.chars() {
                        if c == c1 {
                            let e = step_4.entry(c1).or_insert(0);
                            *e += 1;
                        }
                    }
                }
            }
        }
    }
    result[2] = step_4.iter().min_by_key(|a| a.1).map(|e| *e.0).unwrap();
    result[5] = step_4.iter().max_by_key(|a| a.1).map(|e| *e.0).unwrap();

    result
}

fn get_output_value(outputs: &[String], result: [char; 7]) -> usize {
    let numbers: [Vec<usize>; 10] = [
        vec![0, 1, 2, 4, 5, 6],    // 0
        vec![2, 5],                // 1
        vec![0, 2, 3, 4, 6],       // 2
        vec![0, 2, 3, 5, 6],       // 3
        vec![1, 2, 3, 5],          // 4
        vec![0, 1, 3, 5, 6],       // 5
        vec![0, 1, 3, 4, 5, 6],    // 6
        vec![0, 2, 5],             // 7
        vec![0, 1, 2, 3, 4, 5, 6], // 8
        vec![0, 1, 2, 3, 5, 6],    // 9
    ];

    let mut r = String::new();
    for output in outputs {
        let mut digits = Vec::with_capacity(7);
        for c in output.chars() {
            let index = result.iter().position(|c1| *c1 == c).unwrap();
            digits.push(index);
        }
        digits.sort_unstable();

        let num = numbers.iter().position(|n| *n == digits).unwrap();
        r.push(std::char::from_digit(num as u32, 10).unwrap());
    }

    r.parse().unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(26, part1(get_data("test.txt")));
    assert_eq!(247, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(61229, part2(get_data("test.txt")));
    assert_eq!(933305, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Signal> {
    let mut signals = Vec::new();

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .for_each(|line| {
            let parts = line.split_once(" | ").unwrap();

            let signal_patterns: Vec<String> = parts.0.split_whitespace().into_iter().map(|s| s.to_string()).collect();

            let ouput: Vec<String> = parts.1.split_whitespace().into_iter().map(|s| s.to_string()).collect();

            signals.push(Signal { signal_patterns, ouput });
        });

    signals
}
