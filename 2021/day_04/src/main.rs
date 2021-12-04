use std::fs;

const BINGO_SIZE: usize = 5;

type Board = [[u8; BINGO_SIZE]; BINGO_SIZE];

#[derive(Debug)]
struct Game {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            numbers: vec![],
            boards: vec![],
        }
    }
}

fn main() {
    // println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(game: Game) -> usize {
    0
}

fn part2(lines: Vec<String>) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(4512, part1(get_data("test.txt")));
    // assert_eq!(3895776, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(230, part2(get_data("test.txt")));
    // assert_eq!(7928162, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Game {
    let mut game = Game::default();

    let data: Vec<String> = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect();
    dbg!(&data);

    // Numbers
    // -------

    game
}
