use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Num(usize),
    Add,
    Mul,
    Open,
    Close,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot find input.txt");

    println!("Part 1 result: {}", part1(get_data(&input)));
    println!("Part 2 result: {}", part2(get_data(&input)));
}

fn part1(data: Vec<Vec<Value>>) -> usize {
    calculate_sum(data, "part1")
}

fn part2(data: Vec<Vec<Value>>) -> usize {
    calculate_sum(data, "part2")
}

fn calculate_sum(data: Vec<Vec<Value>>, part: &str) -> usize {
    let mut sum = 0;

    for line in &data {
        let mut new_line = line.clone();

        while new_line.contains(&Value::Open) {
            let mut clone_line = new_line.clone();
            let mut last_opened = Some(0);

            for (index, e) in new_line.iter().enumerate() {
                match e {
                    Value::Open => {
                        last_opened = Some(index);
                    }
                    Value::Close => {
                        if let Some(opened) = last_opened {
                            let sub_line = &new_line[opened + 1..index];
                            if sub_line.contains(&Value::Open) {
                                continue;
                            } else {
                                let val = match part {
                                    "part1" => calculate_expression_without_bracket(sub_line),
                                    "part2" => calculate_expression_without_bracket_2(sub_line),
                                    _ => panic!("Invalid part"),
                                };
                                for _i in 0..index - opened {
                                    clone_line.remove(opened);
                                }
                                clone_line[opened] = Value::Num(val);

                                break;
                            }
                        }
                    }
                    _ => (),
                }
            }

            new_line = clone_line;
        }

        sum += match part {
            "part1" => calculate_expression_without_bracket(&new_line),
            "part2" => calculate_expression_without_bracket_2(&new_line),
            _ => panic!("Invalid part"),
        };
    }

    sum
}

fn calculate_expression_without_bracket(line: &[Value]) -> usize {
    let mut result = 0;

    for (index, value) in line.iter().enumerate() {
        match value {
            Value::Num(v) => {
                if index == 0 {
                    result = *v;
                }
            }
            Value::Add => {
                let next = &line[index + 1];
                let val = match next {
                    Value::Num(n) => n,
                    _ => &0,
                };
                result += *val;
            }
            Value::Mul => {
                let next = &line[index + 1];
                let val = match next {
                    Value::Num(n) => n,
                    _ => &1,
                };
                result *= *val;
            }
            _ => (),
        }
    }

    result
}

fn calculate_expression_without_bracket_2(line: &[Value]) -> usize {
    // On calcule d'abord toutes les additions
    let mut new_line = line.to_vec();
    while new_line.contains(&Value::Add) {
        let mut clone_line = new_line.clone();

        for (index, e) in new_line.iter().enumerate() {
            if let Value::Add = e {
                let prev = match new_line[index - 1] {
                    Value::Num(v) => v,
                    _ => 0,
                };
                let next = match new_line[index + 1] {
                    Value::Num(v) => v,
                    _ => 0,
                };
                let sum = Value::Num(prev + next);

                clone_line.remove(index);
                clone_line.remove(index);
                clone_line[index - 1] = sum;

                break;
            }
        }

        new_line = clone_line;
    }

    // Il ne reste plus que les multiplications
    calculate_expression_without_bracket(&new_line)
}

fn get_data(data: &str) -> Vec<Vec<Value>> {
    data.lines()
        .map(|line| {
            line.replace(" ", "")
                .chars()
                .map(|c| match c {
                    '+' => Value::Add,
                    '*' => Value::Mul,
                    '(' => Value::Open,
                    ')' => Value::Close,
                    _ => Value::Num(c.to_string().parse().unwrap()),
                })
                .collect()
        })
        .collect()
}

#[test]
fn test_get_data() {
    let values = vec![vec![
        Value::Num(1),
        Value::Add,
        Value::Open,
        Value::Num(2),
        Value::Mul,
        Value::Num(3),
        Value::Close,
        Value::Add,
        Value::Open,
        Value::Num(4),
        Value::Mul,
        Value::Open,
        Value::Num(5),
        Value::Add,
        Value::Num(6),
        Value::Close,
        Value::Close,
    ]];
    assert_eq!(values, get_data("1 + (2 * 3) + (4 * (5 + 6))"));

    let values = vec![vec![
        Value::Num(5),
        Value::Add,
        Value::Open,
        Value::Num(8),
        Value::Mul,
        Value::Num(3),
        Value::Add,
        Value::Num(9),
        Value::Add,
        Value::Num(3),
        Value::Mul,
        Value::Num(4),
        Value::Mul,
        Value::Num(3),
        Value::Close,
    ]];
    assert_eq!(values, get_data("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
}

#[test]
fn test_calculate_expression_without_bracket() {
    let data = get_data("1 + 2 * 3 + 4 * 5 + 6");
    let data = &data[0];
    assert_eq!(71, calculate_expression_without_bracket(data));

    let data = get_data("8 * 2 * 5 + 1 + 3 * 6");
    let data = &data[0];
    assert_eq!(504, calculate_expression_without_bracket(data));
}

#[test]
fn test_part1() {
    assert_eq!(51, part1(get_data("1 + (2 * 3) + (4 * (5 + 6))")));
    assert_eq!(26, part1(get_data("2 * 3 + (4 * 5)")));
    assert_eq!(437, part1(get_data("5 + (8 * 3 + 9 + 3 * 4 * 3)")));
    assert_eq!(
        12240,
        part1(get_data("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"))
    );
    assert_eq!(
        13632,
        part1(get_data("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"))
    );

    let input = fs::read_to_string("input.txt").expect("Cannot find input.txt");
    assert_eq!(25190263477788, part1(get_data(&input)));
}

#[test]
fn test_calculate_expression_without_bracket_2() {
    let data = get_data("1 + 2 * 3 + 4 * 5 + 6");
    let data = &data[0];
    assert_eq!(231, calculate_expression_without_bracket_2(data));

    let data = get_data("8 * 2 * 5 + 1 + 3 * 6");
    let data = &data[0];
    assert_eq!(864, calculate_expression_without_bracket_2(data));
}

#[test]
fn test_part2() {
    assert_eq!(51, part2(get_data("1 + (2 * 3) + (4 * (5 + 6))")));
    assert_eq!(46, part2(get_data("2 * 3 + (4 * 5)")));
    assert_eq!(1445, part2(get_data("5 + (8 * 3 + 9 + 3 * 4 * 3)")));
    assert_eq!(
        669060,
        part2(get_data("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"))
    );
    assert_eq!(
        23340,
        part2(get_data("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"))
    );

    let input = fs::read_to_string("input.txt").expect("Cannot find input.txt");
    assert_eq!(297139939002972, part2(get_data(&input)));
}
