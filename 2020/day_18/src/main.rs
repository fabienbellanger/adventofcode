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
}

fn part1(data: Vec<Vec<Value>>) -> usize {
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
                                let val = calculate_expression_without_bracket(sub_line);
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

        sum += calculate_expression_without_bracket(&new_line);
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
