use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Interval {
    min: usize,
    max: usize,
}

fn main() {
    println!("Part 1 result: {}", part1());
    println!("Part 2 result: {}", part2());
}

fn part1() -> usize {
    0
}

fn part2() -> usize {
    0
}

fn get_data() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

fn _get_data_test() {
    let data = fs::read_to_string("test.txt").expect("Cannot read the file test.txt");

    let mut parts = data.split("\n\n");
    let ranges = parts.next().unwrap();
    let your_ticket = parts.next().unwrap();
    let nearby_tickets = parts.next().unwrap();
    dbg!(ranges, your_ticket, nearby_tickets);

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
        
        intervals.push(Interval{
            min: (&cap[1]).parse().unwrap(), 
            max: (&cap[2]).parse().unwrap(),
        });
        intervals.push(Interval{
            min: (&cap[3]).parse().unwrap(), 
            max: (&cap[4]).parse().unwrap(),
        });
    }
    
    // Nearby tickets

}

#[test]
fn test_part1() {
    _get_data_test();
    assert_eq!(71, part1());
    // assert_eq!(4886706177792, part1());
}

// #[test]
// fn test_part2() {
//     assert_eq!(208, part2());
//     assert_eq!(3348493585827, part2());
// }
