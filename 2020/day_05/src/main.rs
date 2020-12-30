use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
}

fn part1(data: Vec<String>) -> usize {
    let mut max = 0;

    for pass in data {
        let seat_id = get_seat_id(pass);

        if seat_id > max {
            max = seat_id;
        }
    }

    max
}

fn get_seat_id(pass: String) -> usize {
    // -------------------------------
    // F => "front"
    // B => "back"
    // L => "left"
    // R => "right"
    // -------------------------------
    // 128 rows and 8 columns
    // -------------------------------
    // 7 first = F or B
    // 3 last  = L or R
    // -------------------------------
    // F means to take the lower half
    // B means to take the upper half
    // L means to keep the lower half
    // R means to keep the upper half
    // -------------------------------
    // seat_id = row * 8 + column
    // -------------------------------

    let mut middle: usize;
    let mut min_row = 0;
    let mut max_row = 127;
    let mut min_col = 0;
    let mut max_col = 7;

    for (i, c) in pass.chars().enumerate() {
        if i < 7 {
            // Row
            middle = (min_row + max_row) / 2;
            
            if c == 'F' {
                // lower half
                max_row = middle;
            } else {
                // upper half
                min_row = middle + 1;
            }
        } else {
            // Column
            middle = (min_col + max_col) / 2;
            
            if c == 'L' {
                // lower half
                max_col = middle;
            } else {
                // upper half
                min_col = middle + 1;
            }
        }
        // dbg!(i, c, middle, min_row, max_row, min_col, max_col);
    }

    min_row * 8 + max_col
}

fn get_data() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

fn _get_data_test() -> Vec<String> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

#[test]
fn test_get_seat_id() {
    assert_eq!(357, get_seat_id(String::from("FBFBBFFRLR")));
    assert_eq!(567, get_seat_id(String::from("BFFFBBFRRR")));
    assert_eq!(119, get_seat_id(String::from("FFFBBBFRRR")));
    assert_eq!(820, get_seat_id(String::from("BBFFBBFRLL")));
}

#[test]
fn test_part1() {
    assert_eq!(820, part1(_get_data_test()));
    assert_eq!(858, part1(get_data()));
}
