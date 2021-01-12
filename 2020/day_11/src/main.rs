use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Occuped,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    // println!("Part 2 result: {}", part2(&mut get_data()));
}

fn part1(seats: Vec<Vec<SeatState>>) -> usize {
    let mut last: Vec<Vec<SeatState>> = seats.clone();
    let mut new_seats: Vec<Vec<SeatState>> = Vec::new();
    loop {
        new_seats.clear();
        for (row_index, row) in seats.iter().enumerate() {
            new_seats.push(Vec::new());
            for (col_index, _col) in row.iter().enumerate() {
                new_seats[row_index].push(apply_rules(&last, row_index, col_index));
            }
        }

        if is_seats_equal(&last, &new_seats) {
            break;
        } else {
            last = new_seats.clone();
        }
    }

    count_occuped(&last)
}

fn is_seats_equal(seats_1: &Vec<Vec<SeatState>>, seats_2: &Vec<Vec<SeatState>>) -> bool {
    for (i, r) in seats_1.iter().enumerate() {
        for (j, _) in r.iter().enumerate() {
            if seats_1[i][j] != seats_2[i][j] {
                return false;
            }
        }
    }
    
    true
}

fn count_occuped(seats: &Vec<Vec<SeatState>>) -> usize {
    let mut n = 0;

    for r in seats {
        for c in r {
            if c == &SeatState::Occuped {
                n += 1;
            }
        }
    }

    n
}

fn apply_rules(seats: &Vec<Vec<SeatState>>, row_index: usize, col_index: usize) -> SeatState {
    match seats[row_index][col_index] {
        SeatState::Floor => SeatState::Floor,
        SeatState::Empty => {
            // If no seat occuped => occuped
            let mut number_of_occuped = 0;
            let mut row_min: isize = -1;
            let mut row_max: isize = 1;
            let mut col_min: isize = -1;
            let mut col_max: isize = 1;

            if row_index == 0 {
                row_min = 0;
            } else if row_index == seats[row_index].len() - 1 {
                row_max = 0
            }

            if col_index == 0 {
                col_min = 0;
            } else if col_index == seats.len() - 1 {
                col_max = 0
            }

            for r in row_min..=row_max {
                if seats.get((row_index as isize + r) as usize).is_some() {
                    let line = seats.get((row_index as isize + r) as usize).unwrap();
                    for c in col_min..=col_max {
                        if r == 0 && c == 0 {
                            continue;
                        }
                        
                        if line.get((col_index as isize + c) as usize).is_some() {
                            match line.get((col_index as isize + c) as usize).unwrap() {
                                SeatState::Occuped => number_of_occuped += 1,
                                _ => (),
                            };
                        }
                    }
                }
            }

            match number_of_occuped {
                0 => SeatState::Occuped,
                _ => SeatState::Empty,
            }
        },
        SeatState::Occuped => {
            // If 4 or more seats occuped => empty
            let mut number_of_occuped = 0;
            let mut row_min: isize = -1;
            let mut row_max: isize = 1;
            let mut col_min: isize = -1;
            let mut col_max: isize = 1;

            if row_index == 0 {
                row_min = 0;
            } else if row_index == seats[row_index].len() - 1 {
                row_max = 0
            }

            if col_index == 0 {
                col_min = 0;
            } else if col_index == seats.len() - 1 {
                col_max = 0
            }

            for r in row_min..=row_max {
                if seats.get((row_index as isize + r) as usize).is_some() {
                    let line = seats.get((row_index as isize + r) as usize).unwrap();
                    for c in col_min..=col_max {
                        if r == 0 && c == 0 {
                            continue;
                        }
                        
                        if line.get((col_index as isize + c) as usize).is_some() {
                            match line.get((col_index as isize + c) as usize).unwrap() {
                                SeatState::Occuped => number_of_occuped += 1,
                                _ => (),
                            };
                        }
                    }
                }
            }

            match number_of_occuped {
                4..=8  => SeatState::Empty,
                _ => SeatState::Occuped,
            }
        }
    }
}

fn get_data() -> Vec<Vec<SeatState>> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c|
                    match c {
                        'L' => SeatState::Empty,
                        '#' => SeatState::Occuped,
                        _ => SeatState::Floor,
                    }
                )
                .collect()
        })
        .collect()
}

fn _get_data_test() -> Vec<Vec<SeatState>> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c|
                    match c {
                        'L' => SeatState::Empty,
                        '#' => SeatState::Occuped,
                        _ => SeatState::Floor,
                    }
                )
                .collect()
        })
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(37, part1(_get_data_test()));
    // assert_eq!(2070, part1(&mut get_data()));
}

// #[test]
// fn test_part2() {
//     let mut test = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
//     assert_eq!(8, part2(&mut test));
//     assert_eq!(19208, part2(&mut _get_data_test()));
//     assert_eq!(24179327893504, part2(&mut get_data()));
// }