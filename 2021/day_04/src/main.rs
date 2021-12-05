use std::fs;

const BINGO_SIZE: usize = 5;

#[derive(Debug, Clone, PartialEq)]
struct Board {
    board: Vec<Vec<(u8, bool)>>,
}

impl Board {
    fn update(&mut self, n: u8) {
        for row in self.board.iter_mut() {
            for item in row.iter_mut() {
                if item.0 == n && !item.1 {
                    item.1 = true;
                }
            }
        }
    }

    fn winned(&self) -> bool {
        // Rows?
        let mut win = false;
        'r: for row in &self.board {
            for item in row {
                if !item.1 {
                    continue 'r;
                }
            }
            // No item equal to false, so it's ok
            win = true;
            break;
        }

        if !win {
            // Columns?
            'c: for row in 0..BINGO_SIZE {
                for col in 0..BINGO_SIZE {
                    if !self.board[col][row].1 {
                        continue 'c;
                    }
                }
                win = true;
                break;
            }
        }

        win
    }
}

#[derive(Debug, Default)]
struct Game {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(game: Game) -> usize {
    let mut boards = game.boards.clone();
    let mut winner_sum: usize = 0;
    let mut winner_number: usize = 0;

    'win: for number in game.numbers {
        for board in &mut boards {
            board.update(number);
            if board.winned() {
                winner_number = number as usize;
                for row in &board.board {
                    for item in row {
                        if !item.1 {
                            winner_sum += item.0 as usize;
                        }
                    }
                }
                break 'win;
            }
        }
    }

    winner_number * winner_sum
}

fn part2(game: Game) -> usize {
    let mut boards = game.boards.clone();
    let mut winner_boards: Vec<Board> = vec![];
    let mut winner_sum: usize = 0;
    let mut winner_number: usize = 0;

    for number in game.numbers {
        for board in &mut boards {
            if !winner_boards.contains(board) {
                board.update(number);
                if board.winned() {
                    winner_number = number as usize;
                    winner_sum = 0;
                    for row in &board.board {
                        for item in row {
                            if !item.1 {
                                winner_sum += item.0 as usize;
                            }
                        }
                    }

                    winner_boards.push(board.clone());
                }
            }
        }
    }

    winner_number * winner_sum
}

#[test]
fn test_winner_board_row() {
    let mut board = Board {
        board: vec![
            vec![(1, false), (4, false), (6, true), (9, false), (9, false)],
            vec![(12, true), (14, true), (16, false), (19, true), (19, true)],
            vec![
                (21, false),
                (24, false),
                (26, true),
                (29, false),
                (29, false),
            ],
            vec![
                (31, false),
                (34, false),
                (36, true),
                (39, false),
                (39, false),
            ],
            vec![
                (41, false),
                (44, false),
                (46, true),
                (49, false),
                (49, false),
            ],
        ],
    };
    assert_eq!(false, board.winned());

    board.update(16);
    assert_eq!(true, board.winned());
}

#[test]
fn test_winner_board_column() {
    let mut board = Board {
        board: vec![
            vec![(1, false), (4, false), (6, true), (9, false), (9, false)],
            vec![(12, true), (14, true), (16, false), (19, true), (19, true)],
            vec![(21, true), (24, false), (26, true), (29, false), (29, true)],
            vec![(31, true), (34, false), (36, true), (39, false), (39, true)],
            vec![(41, true), (44, false), (46, true), (49, false), (49, true)],
        ],
    };
    assert_eq!(false, board.winned());

    board.update(9);
    assert_eq!(true, board.winned());
}

#[test]
fn test_part1() {
    assert_eq!(4512, part1(get_data("test.txt")));
    assert_eq!(21607, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(1924, part2(get_data("test.txt")));
    assert_eq!(19012, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Game {
    let data = fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let (numbers, boards) = data.split_first().expect("error when parsing input data");

    // Numbers
    // -------
    let numbers: Vec<u8> = numbers.split(',').map(|n| n.parse().unwrap()).collect();

    // Boards
    // ------
    let boards: Vec<_> = boards
        .chunks(BINGO_SIZE + 1)
        .map(|board| {
            let board: Vec<_> = board
                .iter()
                .skip(1)
                .map(|s| {
                    s.split_whitespace()
                        .map(|d| (d.parse().unwrap(), false))
                        .collect::<Vec<(u8, bool)>>()
                })
                .collect();
            Board { board }
        })
        .collect();

    Game { numbers, boards }
}
