use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(&mut get_data()));
    println!("Part 2 result: {}", part2(&get_data()));
}

fn part1(data: &mut [usize]) -> usize {
    data.sort_unstable();

    let mut diff_1 = 0;
    let mut diff_3 = 1;

    for (index, current) in data.iter().take(data.len() - 1).enumerate() {
        // Start with 0 value
        if index == 0 {
            match current {
                1 => diff_1 += 1,
                3 => diff_3 += 1,
                _ => (),
            }
        }

        let diff = data[index + 1] - current;
        match diff {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => (),
        }
    }

    diff_1 * diff_3
}

fn part2(data: &[usize]) -> usize {
    0
}

fn get_data() -> Vec<usize> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn _get_data_test() -> Vec<usize> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(220, part1(&mut _get_data_test()));
    // assert_eq!(Some(), part1(&get_data()));
}

#[test]
fn test_part2() {
    // assert_eq!(, part2(&mut _get_data_test()));
    // assert_eq!(, part2(&mut get_data()));
}
