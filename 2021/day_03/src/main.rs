use std::fs;

enum Power {
    Gamma,
    Epsilon,
}

enum Life {
    Oxygen,
    Co2,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(lines: Vec<String>) -> usize {
    let size = lines.first().unwrap().chars().count();
    let mut tab_cmp: Vec<(u32, u32)> = vec![(0, 0); size];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                tab_cmp[i].0 += 1;
            } else {
                tab_cmp[i].1 += 1;
            }
        }
    }

    let gamma: String = calculate_part1(&tab_cmp, &Power::Gamma);
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();

    let epsilon: String = calculate_part1(&tab_cmp, &Power::Epsilon);
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn calculate_part1(tab_cmp: &[(u32, u32)], power: &Power) -> String {
    tab_cmp
        .iter()
        .map(|col| match (power, col) {
            (Power::Gamma, col) if col.0 > col.1 => '0',
            (Power::Epsilon, col) if col.0 <= col.1 => '0',
            _ => '1',
        })
        .collect()
}

fn part2(lines: Vec<String>) -> usize {
    let size = lines.first().unwrap().chars().count();

    let oxygen = calculate_part2(&lines, size, &Life::Oxygen);
    let oxygen = usize::from_str_radix(&oxygen, 2).unwrap();

    let co2 = calculate_part2(&lines, size, &Life::Co2);
    let co2 = usize::from_str_radix(&co2, 2).unwrap();

    oxygen * co2
}

fn calculate_part2(lines: &[String], size: usize, life: &Life) -> String {
    let mut result = lines.to_owned();
    for x in 0..size {
        let mut col = (0, 0);
        for line in &result {
            if line.chars().nth(x).unwrap() == '0' {
                col.0 += 1;
            } else {
                col.1 += 1;
            }
        }

        let start = match (life, col) {
            (Life::Oxygen, col) if col.0 > col.1 => '0',
            (Life::Co2, col) if col.0 <= col.1 => '0',
            _ => '1',
        };

        result = result
            .into_iter()
            .filter(|line| line.chars().nth(x).unwrap() == start)
            .collect();

        // Good line found
        if result.len() == 1 {
            break;
        }
    }

    result.first().unwrap().to_string()
}

#[test]
fn test_part1() {
    assert_eq!(198, part1(get_data("test.txt")));
    assert_eq!(3895776, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(230, part2(get_data("test.txt")));
    assert_eq!(7928162, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect()
}
