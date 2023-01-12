use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

#[derive(Debug)]
struct Valve {
    name: String,
    flow: usize,
    links: Vec<String>,
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
        let links = list.split(", ").into_iter().map(|v| v.to_string()).collect();

        Self {
            name: name.to_owned(),
            flow: flow.parse().unwrap(),
            links,
        }
    }
}

type Path = Vec<(String, String)>;

#[derive(Debug)]
struct Network {
    valves: HashMap<String, Valve>,
}

impl Network {
    fn parse(data: &str) -> Self {
        let valves = data
            .trim()
            .lines()
            .map(|line| {
                let v = Valve::parse(line);
                (v.name.clone(), v)
            })
            .collect();

        Self { valves }
    }

    // From AA:
    // We can get to HH using path [(AA, DD), (DD, EE), (EE, FF), (FF, GG), (GG, HH)]
    // We can get to AA using path []
    // We can get to BB using path [(AA, BB)]
    // etc.
    fn connections(&self, start: String) -> HashMap<String, Path> {
        let mut current: HashMap<String, Path> = HashMap::new();
        current.insert(start, vec![]);

        let mut connections = current.clone();

        while !current.is_empty() {
            let mut next: HashMap<String, Path> = HashMap::new();

            for (name, path) in current {
                for link in self.valves[&name].links.iter().cloned() {
                    if let Entry::Vacant(e) = connections.entry(link.clone()) {
                        let connection_path: Path = path
                            .iter()
                            .cloned()
                            .chain(std::iter::once((name.clone(), link.clone())))
                            .collect();
                        e.insert(connection_path.clone());
                        next.insert(link, connection_path);
                    }
                }
            }

            current = next;
        }

        connections
    }
}

fn part1(network: Network) -> usize {
    println!("From AA:");
    for (name, path) in network.connections("AA".to_owned()) {
        println!("We can get to {name} using path {path:?}");
    }

    0
}

fn part2(_network: Network) -> usize {
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

fn get_data(file: &str) -> Network {
    let data = fs::read_to_string(file).expect("Cannot read the file input.txt");

    Network::parse(&data)
}
