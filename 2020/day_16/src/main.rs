use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Interval {
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct Note {
    intervals: Vec<Interval>,
    ticket: Vec<usize>,
    nearby_tickets: Vec<usize>,
}

impl Note {
    fn new(intervals: Vec<Interval>, ticket: Vec<usize>, nearby_tickets: Vec<usize>) -> Self {
        Self {
            intervals,
            ticket,
            nearby_tickets,
        }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    // println!("Part 2 result: {}", part2());
}

fn part1(note: Note) -> usize {
    dbg!(note);
    0
}

fn part2() -> usize {
    0
}

fn get_data() -> Note {
    let data = fs::read_to_string("input.txt").expect("Cannot read the file input.txt");

    get_note(data)
}

fn _get_data_test() -> Note {
    let data = fs::read_to_string("test.txt").expect("Cannot read the file test.txt");
    get_note(data)
}

fn get_note(data: String) -> Note {
    let mut parts = data.split("\n\n");
    let ranges = parts.next().unwrap();
    let _your_ticket = parts.next().unwrap();
    let nearby_tickets = parts.next().unwrap();

    // Ranges
    let ranges_regex = Regex::new(r"^\w+: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut intervals: Vec<Interval> = Vec::new();
    for line in ranges.trim().lines() {
        if !ranges_regex.is_match(line) {
            panic!("Invalid range");
        }

        let cap = ranges_regex.captures(line).unwrap();
        if cap.len() != 5 {
            panic!("Invalid range");
        }

        intervals.push(Interval {
            min: (&cap[1]).parse().unwrap(),
            max: (&cap[2]).parse().unwrap(),
        });
        intervals.push(Interval {
            min: (&cap[3]).parse().unwrap(),
            max: (&cap[4]).parse().unwrap(),
        });
    }

    // Your ticket
    let ticket: Vec<usize> = Vec::new();

    // Nearby tickets
    let mut numbers: Vec<usize> = Vec::new();
    for line in nearby_tickets.trim().lines().skip(1) {
        let mut n: Vec<usize> = line.split(',').map(|n| n.parse().unwrap()).collect();
        numbers.append(&mut n);
    }

    Note::new(intervals, ticket, numbers)
}

#[test]
fn test_part1() {
    assert_eq!(71, part1(_get_data_test()));
    // assert_eq!(4886706177792, part1());
}

// #[test]
// fn test_part2() {
//     assert_eq!(208, part2());
//     assert_eq!(3348493585827, part2());
// }
