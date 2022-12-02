use std::fs;

#[derive(Debug)]
enum Game {
    Win,
    Lose,
    EndDraw,
}

#[derive(Debug)]
struct HandShape {
    name: char,
    value: usize,
    result: Option<Game>,
}

impl HandShape {
    fn new(name: char, value: usize, result: Option<Game>) -> Self {
        Self { name, value, result }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

// Rock > Scissor > Paper > Rock

fn part1(data: Vec<(HandShape, HandShape)>) -> usize {
    data.into_iter()
        .map(|(p1, p2)| match (p1.name, p2.name) {
            ('S', 'R') | ('P', 'S') | ('R', 'P') => 6 + p2.value,
            ('R', 'S') | ('S', 'P') | ('P', 'R') => 0 + p2.value,
            _ => 3 + p2.value,
        })
        .sum::<usize>()
}

fn part2(data: Vec<(HandShape, HandShape)>) -> usize {
    data.into_iter()
        .map(|(p1, p2)| match (p1.name, p2.result) {
            ('R', Some(Game::Lose)) => 3,
            ('P', Some(Game::Lose)) => 1,
            ('S', Some(Game::Lose)) => 2,
            ('R', Some(Game::EndDraw)) => 1 + 3,
            ('P', Some(Game::EndDraw)) => 2 + 3,
            ('S', Some(Game::EndDraw)) => 3 + 3,
            ('R', Some(Game::Win)) => 2 + 6,
            ('P', Some(Game::Win)) => 3 + 6,
            ('S', Some(Game::Win)) => 1 + 6,
            _ => panic!(),
        })
        .sum::<usize>()
}

#[test]
fn test_part1() {
    assert_eq!(15, part1(get_data("test.txt")));
    assert_eq!(13446, part1(get_data("input.txt"))); // 12936
}

#[test]
fn test_part2() {
    assert_eq!(12, part2(get_data("test.txt")));
    assert_eq!(13509, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<(HandShape, HandShape)> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .into_iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let p1 = match parts.next().unwrap() {
                "A" => HandShape::new('R', 1, None),
                "B" => HandShape::new('P', 2, None),
                "C" => HandShape::new('S', 3, None),
                _ => panic!("invalid entry"),
            };
            let p2 = match parts.next().unwrap() {
                "X" => HandShape::new('R', 1, Some(Game::Lose)),
                "Y" => HandShape::new('P', 2, Some(Game::EndDraw)),
                "Z" => HandShape::new('S', 3, Some(Game::Win)),
                _ => panic!("invalid entry"),
            };
            (p1, p2)
        })
        .collect()
}
