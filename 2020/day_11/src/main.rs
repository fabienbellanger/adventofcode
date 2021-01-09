use std::fs;

#[derive(Debug)]
enum SeatState {
    Floor,
    Empty,
    Occuped,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    // println!("Part 2 result: {}", part2(&mut get_data()));
} 

fn part1(data: Vec<Vec<SeatState>>) -> usize {
    dbg!(data);
    0
}

fn get_data() -> Vec<Vec<SeatState>> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| 
            l.chars()
            .map(|s| match s {
                'L' => SeatState::Empty,
                '#' => SeatState::Occuped,
                _ => SeatState::Floor,
            })
            .collect())
        .collect()
}

fn _get_data_test() -> Vec<Vec<SeatState>> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| 
            l.chars()
            .map(|s| match s {
                'L' => SeatState::Empty,
                '#' => SeatState::Occuped,
                _ => SeatState::Floor,
            })
            .collect())
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
