use std::fs;

#[derive(Debug)]
struct Valve {
    name: String,
    flow: usize,
    list: Vec<String>,
}

impl Valve {
    fn parse(data: &str) -> Self {
        let data = data.strip_prefix("Valve ").unwrap();
        let (name, data) = data.split_once(" has flow rate=").unwrap();
        let (flow, list) = if data.contains("tunnels") {
            data.split_once("; tunnels lead to valves ").unwrap()
        } else {
            data.split_once("; tunnel leads to valve ").unwrap()
        };
        let list = list.split(", ").into_iter().map(|v| v.to_string()).collect();

        Self {
            name: name.to_owned(),
            flow: flow.parse().unwrap(),
            list,
        }
    }
}

fn part1(data: Vec<Valve>) -> usize {
    dbg!(&data);
    0
}

fn part2(_data: Vec<Valve>) -> usize {
    0
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("day_16/input.txt")));
    println!("Part 2 result: {}", part2(get_data("day_16/input.txt")));
}

#[test]
fn test_part1() {
    assert_eq!(1651, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(0, part2(get_data("test.txt")));
    // assert_eq!(0, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Valve> {
    let valves = fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(Valve::parse)
        .collect();

    valves
}
