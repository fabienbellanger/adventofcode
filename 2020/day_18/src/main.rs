use std::fs;

#[derive(Debug, PartialEq)]
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
    dbg!(&data.len());
    0
}

fn get_data(data: &str) -> Vec<Vec<Value>> {
    data
        .lines()
        .map(|line| line
            .replace(" ", "")
            .chars()
            .map(|c| match c {
                '+' => Value::Add,
                '*' => Value::Mul,
                '(' => Value::Open,
                ')' => Value::Close,
                _ => Value::Num(c.to_string().parse().unwrap()),
            })
            .collect())
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
fn test_part1() {
    assert_eq!(51, part1(get_data("1 + (2 * 3) + (4 * (5 + 6))")));
    assert_eq!(26, part1(get_data("2 * 3 + (4 * 5)")));
    assert_eq!(437, part1(get_data("5 + (8 * 3 + 9 + 3 * 4 * 3)")));
    assert_eq!(12240, part1(get_data("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")));
    assert_eq!(13632, part1(get_data("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")));
}
