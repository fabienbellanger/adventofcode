use regex::Regex;
use std::fs;

type Ticket = Vec<usize>;

#[derive(Debug)]
struct Interval {
    min: usize,
    max: usize,
}

#[derive(Debug)]
struct Note {
    intervals: Vec<Interval>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Note {
    fn new(intervals: Vec<Interval>, ticket: Ticket, nearby_tickets: Vec<Ticket>) -> Self {
        Self {
            intervals,
            ticket,
            nearby_tickets,
        }
    }

    fn is_ticket_values_in_intervals(&self, value: usize) -> bool {
        for interval in &self.intervals {
            if value >= interval.min && value <= interval.max {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(note: Note) -> usize {
    let mut incorrect_values = Vec::new();
    for ticket in &note.nearby_tickets {
        for value in ticket {
            if !note.is_ticket_values_in_intervals(*value) {
                incorrect_values.push(*value);
            }
        }
    }

    incorrect_values.iter().sum()
}

fn part2(_note: Note) -> usize {
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
    let ticket_str = parts.next().unwrap();
    let nearby_tickets_str = parts.next().unwrap();

    // Ranges
    let ranges_regex = Regex::new(r"^[\w ]+: (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut intervals: Vec<Interval> = Vec::new();
    for line in ranges.trim().lines() {
        if !ranges_regex.is_match(line) {
            panic!("Invalid range");
        }

        let cap = ranges_regex.captures(line).unwrap();
        if cap.len() != 5 {
            panic!("Invalid range (match len != 5)");
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
    let ticket: Ticket = ticket_str
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // Nearby tickets
    let mut nearby_tickets: Vec<Ticket> = Vec::new();
    for line in nearby_tickets_str.trim().lines().skip(1) {
        let ticket: Ticket = line.split(',').map(|n| n.parse().unwrap()).collect();
        nearby_tickets.push(ticket);
    }

    Note::new(intervals, ticket, nearby_tickets)
}

#[test]
fn test_part1() {
    assert_eq!(71, part1(_get_data_test()));
    assert_eq!(21978, part1(get_data()));
}

#[test]
fn test_part2() {
    // assert_eq!(208, part2(_get_data_test()));
    // assert_eq!(3348493585827, part2());
}
