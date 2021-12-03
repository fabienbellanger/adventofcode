use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(lines: Vec<String>) -> usize {
    let size = lines.first().unwrap().chars().count();
    let mut tab: Vec<(u32, u32)> = vec![(0, 0); size];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                tab[i].0 += 1;
            } else {
                tab[i].1 += 1;
            }
        }
    }

    let gamma: String = tab
        .iter()
        .map(|col| match col {
            (col_0, col_1) if col_0 >= col_1 => "0".to_owned(),
            _ => "1".to_owned(),
        })
        .collect();

    let epsilon: String = tab
        .iter()
        .map(|col| match col {
            (col_0, col_1) if col_0 >= col_1 => "1".to_owned(),
            _ => "0".to_owned(),
        })
        .collect();

    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn part2(lines: Vec<String>) -> usize {
    let size = lines.first().unwrap().chars().count();

    let oxygen = calculate_part2(&lines, size, '1');
    let oxygen = usize::from_str_radix(&oxygen, 2).unwrap();

    let co2 = calculate_part2(&lines, size, '0');
    let co2 = usize::from_str_radix(&co2, 2).unwrap();

    oxygen * co2
}

fn calculate_part2(lines: &[String], size: usize, digit: char) -> String {
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

        let start = match (digit, col) {
            ('1', col) if col.0 > col.1 => '0',
            ('0', col) if col.0 <= col.1 => '0',
            _ => '1',
        };

        result = result
            .into_iter()
            .filter(|line| line.chars().nth(x).unwrap() == start)
            .collect();

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
