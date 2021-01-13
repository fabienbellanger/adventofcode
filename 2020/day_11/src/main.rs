use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Occuped,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(seats: Vec<Vec<SeatState>>) -> usize {
    let mut last: Vec<Vec<SeatState>> = seats.clone();
    let mut new_seats: Vec<Vec<SeatState>> = Vec::new();
    loop {
        new_seats.clear();
        for (row_index, row) in seats.iter().enumerate() {
            new_seats.push(Vec::new());
            for (col_index, _col) in row.iter().enumerate() {
                new_seats[row_index].push(apply_rules1(&last, row_index, col_index));
            }
        }

        if is_seats_equal(&last, &new_seats) {
            break;
        } else {
            last = new_seats.clone();
        }

        // _display_seats(&last);
    }

    count_occuped(&last)
}

fn part2(seats: Vec<Vec<SeatState>>) -> usize {
    let mut last: Vec<Vec<SeatState>> = seats.clone();
    let mut new_seats: Vec<Vec<SeatState>> = Vec::new();
    loop {
        new_seats.clear();
        for (row_index, row) in seats.iter().enumerate() {
            new_seats.push(Vec::new());
            for (col_index, _col) in row.iter().enumerate() {
                println!("=====> ({}, {})", row_index, col_index);
                new_seats[row_index].push(apply_rules2(&last, row_index, col_index));
            }
        }

        if is_seats_equal(&last, &new_seats) {
            break;
        } else {
            last = new_seats.clone();
        }

        _display_seats(&last);
    }

    count_occuped(&last)
}

fn _display_seats(seats: &Vec<Vec<SeatState>>) {
    let mut s = String::new();
    for r in seats {
        for c in r {
            match c {
                SeatState::Empty => s.push('L'),
                SeatState::Occuped => s.push('#'),
                SeatState::Floor => s.push('.'),
            };
        }
        s.push('\n');
    }
    println!("{}", s);
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

fn get_bounds(seats: &Vec<Vec<SeatState>>, row_index: usize, col_index: usize) -> (isize, isize, isize, isize) {
    let mut row_min: isize = -1;
    let mut row_max: isize = 1;
    let mut col_min: isize = -1;
    let mut col_max: isize = 1;

    if row_index == 0 {
        row_min = 0;
    } else if row_index == seats.len() - 1 {
        row_max = 0
    }

    if col_index == 0 {
        col_min = 0;
    } else if col_index == seats[row_index].len() - 1 {
        col_max = 0
    }

    (row_min, row_max, col_min, col_max)
}

fn get_bounds2(seats: &Vec<Vec<SeatState>>, row_index: usize, col_index: usize) -> (isize, isize, isize, isize) {
    let rows_number = seats.len();
    let cols_number = seats[row_index].len();

    // let mut row_min: isize = -(rows_number as isize - 1);
    // let mut row_max: isize = rows_number as isize - 1;
    // let mut col_min: isize = -(cols_number as isize - 1);
    // let mut col_max: isize = cols_number as isize - 1;

    // if row_index == 0 {
    //     row_min = 0;
    // } else if row_index == rows_number - 1 {
    //     row_max = 0
    // }

    // if col_index == 0 {
    //     col_min = 0;
    // } else if col_index == cols_number - 1 {
    //     col_max = 0
    // }

    let row_min = -(row_index as isize);
    let row_max = (rows_number as isize) - 1 - (row_index as isize);
    let col_min = -(col_index as isize);
    let col_max = (cols_number as isize) - 1 - (col_index as isize);

    (row_min, row_max, col_min, col_max)
}

fn apply_rules1(seats: &Vec<Vec<SeatState>>, row_index: usize, col_index: usize) -> SeatState {
    match seats[row_index][col_index] {
        SeatState::Floor => SeatState::Floor,
        SeatState::Empty | SeatState::Occuped => {
            let mut number_of_occuped = 0;
            let (row_min, row_max, col_min, col_max) = get_bounds(&seats, row_index, col_index);

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

            match seats[row_index][col_index] {
                SeatState::Empty => match number_of_occuped {
                    0 => SeatState::Occuped,
                    _ => SeatState::Empty,
                },
                SeatState::Occuped => 
                match number_of_occuped {
                    4..=8  => SeatState::Empty,
                    _ => SeatState::Occuped,
                },
                SeatState::Floor => SeatState::Floor,
            }
        }
    }
}

fn apply_rules2(seats: &Vec<Vec<SeatState>>, row_index: usize, col_index: usize) -> SeatState {
    match seats[row_index][col_index] {
        SeatState::Floor => SeatState::Floor,
        SeatState::Empty | SeatState::Occuped => {
            let mut number_of_occuped = 0;
            let (row_min, row_max, col_min, col_max) = get_bounds2(&seats, row_index, col_index);
            // println!("=> ({}, {}) || {} | {} | {} | {}", row_index, col_index, row_min, row_max, col_min, col_max);
            
            // (L, TL, T, TR, R, RB, B, BL)
            let mut directions: (usize, usize, usize, usize, usize, usize, usize, usize) = (0, 0, 0, 0, 0, 0, 0, 0);

            // for r in row_min..=row_max {
            //     if seats.get((row_index as isize + r) as usize).is_some() {
            //         let line = seats.get((row_index as isize + r) as usize).unwrap();
            //         for c in col_min..=col_max {
            //             if r == 0 && c == 0 {
            //                 continue;
            //             }

            //             // Row
            //             if r == 0 {
            //                 // println!("Row ({}, {})", r, c);
            //                 match line.get((col_index as isize + c) as usize).unwrap() {
            //                     &SeatState::Occuped | &SeatState::Empty => {
            //                         if c < 0 {
            //                             // Left
            //                             if directions.0 == 0 {
            //                                 directions.0 = 1;
            //                                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                                     println!("L");
            //                                     number_of_occuped += 1;
            //                                 }
            //                             }
            //                         } else {
            //                             // Right
            //                             if directions.4 == 0 {
            //                                 directions.4 = 1;
            //                                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                                     println!("R");
            //                                     number_of_occuped += 1;
            //                                 }
            //                             }
            //                         }
            //                     },
            //                     &SeatState::Floor => (),
            //                 };
            //             }

            //             // Column
            //             if c == 0 {
            //                 // println!("Column ({}, {})", r, c);
            //                 match line.get((col_index as isize + c) as usize).unwrap() {
            //                     &SeatState::Occuped | &SeatState::Empty => {
            //                         if r < 0 {
            //                             // Left
            //                             if directions.2 == 0 {
            //                                 directions.2 = 1;
            //                                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                                     println!("T");
            //                                     number_of_occuped += 1;
            //                                 }
            //                             }
            //                         } else {
            //                             // Right
            //                             if directions.6 == 0 {
            //                                 directions.6 = 1;
            //                                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                                     println!("B");
            //                                     number_of_occuped += 1;
            //                                 }
            //                             }
            //                         }
            //                     },
            //                     &SeatState::Floor => (),
            //                 };
            //             }

            //             // Diagonales
            //             if isize::abs(r) == isize::abs(c) {
            //                 // println!("Diagonal ({}, {})", r, c);
            //                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                     if c < 0 {
            //                         // Left
            //                         if r < 0 {
            //                             // Left Top
            //                             if directions.1 == 0 {
            //                                 directions.1 = 1;
            //                                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                                     println!("LT");
            //                                     number_of_occuped += 1;
            //                                 }
            //                             }
            //                         } else {
            //                             // Left Bottom
            //                             if directions.7 == 0 {
            //                                 directions.7 = 1;
            //                                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                                     println!("LB");
            //                                     number_of_occuped += 1;
            //                                 }
            //                             }
            //                         }
            //                     } else {
            //                         // Right
            //                         if r < 0 {
            //                             // Right Top
            //                             if directions.3 == 0 {
            //                                 directions.3 = 1;
            //                                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                                     println!("RT");
            //                                     number_of_occuped += 1;
            //                                 }
            //                             }
            //                         } else {
            //                             // Right Bottom
            //                             if directions.5 == 0 {
            //                                 directions.5 = 1;
            //                                 if line.get((col_index as isize + c) as usize).unwrap() == &SeatState::Occuped {
            //                                     println!("RB");
            //                                     number_of_occuped += 1;
            //                                 }
            //                             }
            //                         }
            //                     }
            //                 }
            //             }
                        
            //             // if line.get((col_index as isize + c) as usize).is_some() {
            //             //     match line.get((col_index as isize + c) as usize).unwrap() {
            //             //         SeatState::Occuped => number_of_occuped += 1,
            //             //         _ => (),
            //             //     };
            //             // }
            //         }
            //     }
            // }

            // Left
            let line = seats.get(row_index).unwrap();
            for c in 1..col_min+1 {
                let seat = line.get((col_index as isize - c) as usize).unwrap();
                if seat == &SeatState::Empty {
                    break;
                } else if seat == &SeatState::Occuped {
                    number_of_occuped += 1;
                } else {
                    continue;
                }
            } 

            dbg!(number_of_occuped);

            match seats[row_index][col_index] {
                SeatState::Empty => match number_of_occuped {
                    0 => SeatState::Occuped,
                    _ => SeatState::Empty,
                },
                SeatState::Occuped => 
                match number_of_occuped {
                    5..=8  => SeatState::Empty,
                    _ => SeatState::Occuped,
                },
                SeatState::Floor => SeatState::Floor,
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
    assert_eq!(2222, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(26, part2(_get_data_test()));
    // assert_eq!(, part2(get_data()));
}
