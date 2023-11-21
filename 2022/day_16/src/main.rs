use std::collections::HashSet;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Move {
    reward: usize,
    target: String,
    path: Path,
}

impl Move {
    fn cost(&self) -> usize {
        let travel_turn = self.path.len();
        let open_turn = 1;
        travel_turn + open_turn
    }
}

#[derive(Clone)]
struct State<'a> {
    net: &'a Network,
    position: String,
    max_turns: usize,
    turn: usize,
    pressure: usize,
    open_valves: HashSet<String>,
}

impl State<'_> {
    fn turns_left(&self) -> usize {
        self.max_turns - self.turn
    }

    /// Compute all moves and expected reward (pressure contributed till time
    /// runs out if we travel to it and open it now)
    fn moves(&self) -> Vec<Move> {
        self.net
            .connections(self.position.clone())
            .into_iter()
            .filter_map(|(name, path)| {
                if self.open_valves.contains(&name) {
                    return None;
                }

                let flow = self.net.valves[&name].flow;
                if flow == 0 {
                    return None;
                }

                let travel_turns = path.len();
                let open_turns = 1;
                let turns_spent_open = self.turns_left().checked_sub(travel_turns + open_turns)?;
                let reward = flow * turns_spent_open;
                Some(Move {
                    reward,
                    target: name,
                    path,
                })
            })
            .collect()
    }

    /// Apply a given move
    fn apply(&self, mv: &Move) -> Self {
        let mut next = self.clone();
        next.position = mv.target.clone();
        next.turn += mv.cost();
        next.pressure += mv.reward;
        next.open_valves.insert(mv.target.clone());
        next
    }

    fn find_best_moves(&self) -> (Self, Vec<Move>) {
        let mut best_moves = vec![];
        let mut best_state = self.clone();
        let mut best_pressure = 0;

        let mut moves = self.moves();
        moves.sort_by_key(|mv| mv.reward);
        moves.reverse();

        for mv in moves {
            let next = self.apply(&mv);
            let (next, mut next_moves) = next.find_best_moves();
            next_moves.push(mv);
            if next.pressure > best_pressure {
                best_pressure = next.pressure;
                best_moves = next_moves;
                best_state = next;
            }
        }

        (best_state, best_moves)
    }
}

fn part1(network: Network) -> usize {
    let state = State {
        net: &network,
        position: "AA".to_owned(),
        max_turns: 30,
        turn: 0,
        open_valves: Default::default(),
        pressure: 0,
    };

    let (state, moves) = state.find_best_moves();
    println!("moves = {:?}, final pressure = {}", moves, state.pressure);

    state.pressure
}

fn part2(_network: Network) -> usize {
    0
}

fn main() {
    let data = get_data("day_16/input.txt");
    println!("Part 1 result: {}", part1(data.clone()));
    println!("Part 2 result: {}", part2(data));
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
