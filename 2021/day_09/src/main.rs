use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn find_low_points(data: &[Vec<u8>]) -> HashMap<(usize, usize), u8> {
    let row_max = data.len();
    let cols_max = data.first().unwrap().len();

    let mut low_points: HashMap<(usize, usize), u8> = HashMap::new();

    for (row_index, row) in data.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
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
    let low_points = find_low_points(&data);

    low_points
        .into_iter()
        .map(|(_index, value)| value as usize + 1)
        .sum()
}

fn part2(data: Vec<Vec<u8>>) -> usize {
    let low_points = find_low_points(&data);
    let mut basins: HashMap<(usize, usize), u8> = HashMap::new();
    let mut basin_number = 0;
    let row_max = data.len();
    let cols_max = data.first().unwrap().len();

    for (low_point, _) in low_points {
        let mut points = vec![low_point];

        while let Some(point) = points.pop() {
            basins.insert(point, basin_number);

            let height = data[point.0][point.1];
            if point.0 > 0
                && data[point.0 - 1][point.1] > height
                && data[point.0 - 1][point.1] != 9
                && !basins.contains_key(&(point.0 - 1, point.1))
            {
                points.push((point.0 - 1, point.1));
            }

            if point.0 < row_max - 1
                && data[point.0 + 1][point.1] > height
                && data[point.0 + 1][point.1] != 9
                && !basins.contains_key(&(point.0 + 1, point.1))
            {
                points.push((point.0 + 1, point.1));
            }

            if point.1 > 0
                && data[point.0][point.1 - 1] > height
                && data[point.0][point.1 - 1] != 9
                && !basins.contains_key(&(point.0, point.1 - 1))
            {
                points.push((point.0, point.1 - 1));
            }

            if point.1 < cols_max - 1
                && data[point.0][point.1 + 1] > height
                && data[point.0][point.1 + 1] != 9
                && !basins.contains_key(&(point.0, point.1 + 1))
            {
                points.push((point.0, point.1 + 1));
            }
        }

        basin_number += 1;
    }

    let mut basins_sum = vec![0; basin_number as usize];
    for (_, basin) in basins {
        basins_sum[basin as usize] += 1;
    }
    basins_sum.sort_unstable();
    basins_sum.reverse();
    basins_sum.into_iter().take(3).product()
}

#[test]
fn test_part1() {
    assert_eq!(15, part1(get_data("test.txt")));
    assert_eq!(558, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(1134, part2(get_data("test.txt")));
    assert_eq!(882942, part2(get_data("input.txt")));
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
