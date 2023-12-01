use std::collections::HashMap;
use utils::data::file_to_vec_string;

const INPUT: &str = "day_01/input.txt";

fn main() {
    println!("Part 1 result: {}", part1(file_to_vec_string(INPUT)));
    println!("Part 2 result: {}", part2(file_to_vec_string(INPUT)));
}

fn part1(data: Vec<String>) -> u32 {
    data.into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .filter_map(|l| format!("{}{}", l[0], l[l.len() - 1]).parse::<u32>().ok())
        .sum()
}

fn convert_str_to_num(line: &str) -> String {
    let numbers = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut tmp = String::new();
    let mut new_line = String::new();
    for c in line.chars() {
        if c.is_ascii_digit() {
            new_line.push(c);
            tmp.clear();
            continue;
        }
        tmp.push(c);

        for (str_num, num) in numbers.clone() {
            if tmp.contains(str_num) {
                new_line.push_str(num);
                tmp.clear();

                // On remet la dernière lettre car il peut y avoir les noms "collés"
                // Ex : "eightwo" => "82"
                tmp.push(c);

                break;
            }
        }
    }

    new_line
}

fn part2(data: Vec<String>) -> u32 {
    part1(data.iter().map(|line| convert_str_to_num(line)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1: &str = "day_01/test1.txt";
    const TEST_2: &str = "day_01/test2.txt";

    #[test]
    fn test_part1() {
        assert_eq!(142, part1(file_to_vec_string(TEST_1)));
        assert_eq!(54390, part1(file_to_vec_string(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(281, part2(file_to_vec_string(TEST_2)));
        assert_eq!(54277, part2(file_to_vec_string(INPUT)));
    }

    #[test]
    fn test_convert_str_to_num() {
        assert_eq!(convert_str_to_num("9seven453seventhree67oneightv"), "97453736718");
        assert_eq!(convert_str_to_num("two1nine"), "219");
        assert_eq!(convert_str_to_num("eightwothree"), "823");
        assert_eq!(convert_str_to_num("7pqrstsixteen"), "76");
    }
}
