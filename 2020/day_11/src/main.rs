use std::fs;

#[derive(Debug)]
enum SeatState {
    Floor,
    Empty,
    Occuped,
}

#[derive(Debug)]
struct SeatsLayout {
    lines_number: usize,
    seats: Vec<SeatState>,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    // println!("Part 2 result: {}", part2(&mut get_data()));
}

fn part1(seats: SeatsLayout) -> usize {
    dbg!(&seats);
    
    for i in 0..seats.seats.len() {
        apply_rules(&seats, i);
    }

    0
}

fn apply_rules(seats: &SeatsLayout, index: usize) -> SeatState {
    let max_seats = seats.seats.len();
    dbg!(max_seats);

    SeatState::Empty
}

fn get_data() -> SeatsLayout {
    let data = fs::read_to_string("input.txt").expect("Cannot read the file input.txt");
    let lines = data.trim().lines();
    let s: Vec<SeatState> = data
        .trim()
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            'L' => SeatState::Empty,
            '#' => SeatState::Occuped,
            _ => SeatState::Floor,
        })
        .collect();

    SeatsLayout {
        lines_number : lines.count(),
        seats: s,
    }
}

fn _get_data_test() -> SeatsLayout {
    let data = fs::read_to_string("test.txt").expect("Cannot read the file test.txt");
    let lines = data.trim().lines();
    let s: Vec<SeatState> = data
        .trim()
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| {
            dbg!(c);
            match c {
                'L' => SeatState::Empty,
                '#' => SeatState::Occuped,
                _ => SeatState::Floor,
            }
        })
        .collect();

    SeatsLayout {
        lines_number : lines.count(),
        seats: s,
    }
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
