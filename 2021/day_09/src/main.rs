use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn find_low_points(data: Vec<Vec<u8>>) -> HashMap<(usize, usize), u8> {
    let row_max = data.len();
    let cols_max = data.first().unwrap().len();

    let mut low_points: HashMap<(usize, usize), u8> = HashMap::new();

    for (row_index, row) in data.iter().enumerate() {
        for (col_index, col) in row.into_iter().enumerate() {
            if row_index > 0 && col >= &data[row_index - 1][col_index]
                || row_index < row_max - 1 && col >= &data[row_index + 1][col_index]
                || col_index > 0 && col >= &data[row_index][col_index - 1]
                || col_index < cols_max - 1 && col >= &data[row_index][col_index + 1]
            {
                continue;
            }
            low_points.insert((row_index, col_index), *col);
        }
    }

    low_points
}

fn part1(data: Vec<Vec<u8>>) -> usize {
    let low_points = find_low_points(data);

    low_points
        .into_iter()
        .map(|(_index, value)| value as usize + 1)
        .sum()
}

fn part2(data: Vec<Vec<u8>>) -> usize {
    let low_points = find_low_points(data);

    let mut bassins: HashMap<(usize, usize), u8> = HashMap::new();

    dbg!(&bassins);

    0
}

#[test]
fn test_part1() {
    assert_eq!(15, part1(get_data("test.txt")));
    assert_eq!(558, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(1134, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|n| {
            n.chars()
                .map(|d| d.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}
