use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Range,
};

type Ticket = Vec<usize>;

#[derive(Debug, Clone, PartialEq)]
struct Ranges {
    range1: Range<usize>,
    range2: Range<usize>,
}

#[derive(Debug)]
struct Note {
    fields: HashMap<String, Ranges>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Note {
    fn new(fields: HashMap<String, Ranges>, ticket: Ticket, nearby_tickets: Vec<Ticket>) -> Self {
        Self {
            fields,
            ticket,
            nearby_tickets,
        }
    }

    fn is_ticket_values_in_fields(&self, value: usize) -> bool {
        for (_name, ranges) in &self.fields {
            if ranges.range1.contains(&value) || ranges.range2.contains(&value) {
                return true;
            }
        }
        false
    }

    fn is_ticket_values_for_field(&self, field_name: &String, value: usize) -> bool {
        let field = self.fields.get(field_name).unwrap();
        if field.range1.contains(&value) || field.range2.contains(&value) {
            return true;
        }
        false
    }

    fn is_ticket_in_fields(&self, ticket: &Ticket) -> bool {
        for value in ticket {
            if !self.is_ticket_values_in_fields(*value) {
                return false;
            }
        }
        true
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
            if !note.is_ticket_values_in_fields(*value) {
                incorrect_values.push(*value);
            }
        }
    }

    incorrect_values.iter().sum()
}

fn part2(note: Note) -> usize {
    let mut nearby_tickets: Vec<&Ticket> = Vec::new();
    for ticket in &note.nearby_tickets {
        if note.is_ticket_in_fields(&ticket) {
            nearby_tickets.push(ticket);
        }
    }

    let ordered_fields = get_ordered_fields(&note, nearby_tickets);
    ordered_fields
        .iter()
        .filter(|(_value, name)| name.contains("departure"))
        .map(|(value, _name)| note.ticket[*value])
        .product()
}

fn get_ordered_fields(note: &Note, tickets: Vec<&Ticket>) -> HashMap<usize, String> {
    let mut solution: HashMap<usize, String> = HashMap::new();
    let mut solution_names: HashSet<String> = HashSet::new();
    let fields_number = note.fields.len();
    let fields = &note.fields;

    let mut loops = 0;
    while solution.len() < fields_number && loops < fields_number.pow(2) {
        for (name, _ranges) in fields {
            if solution_names.contains(name) {
                continue;
            }

            let mut column = 0usize;
            let mut found = false;
            let mut found_more = false;

            for col in 0..fields_number {
                if solution.contains_key(&col) {
                    continue;
                }

                let valid_tickets = tickets.iter().all(|ticket| {
                    note.is_ticket_values_for_field(name, *(ticket.get(col).unwrap()))
                });

                if valid_tickets {
                    if found {
                        found_more = true;
                        break;
                    }

                    found = true;
                    column = col;
                }
            }

            if found && !found_more {
                solution.insert(column, name.clone());
                solution_names.insert(name.clone());
            }
        }

        loops += 1;
    }

    solution
}

fn get_data() -> Note {
    let data = fs::read_to_string("input.txt").expect("Cannot read the file input.txt");
    get_note(data)
}

fn _get_data_test() -> Note {
    let data = fs::read_to_string("test.txt").expect("Cannot read the file test.txt");
    get_note(data)
}

fn _get_data_test2() -> Note {
    let data = fs::read_to_string("test2.txt").expect("Cannot read the file test2.txt");
    get_note(data)
}

fn get_note(data: String) -> Note {
    let mut parts = data.split("\n\n");
    let ranges = parts.next().unwrap();
    let ticket_str = parts.next().unwrap();
    let nearby_tickets_str = parts.next().unwrap();

    // Fields
    let ranges_regex = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut fields: HashMap<String, Ranges> = HashMap::new();
    for line in ranges.trim().lines() {
        if !ranges_regex.is_match(line) {
            panic!("Invalid range");
        }

        let cap = ranges_regex.captures(line).unwrap();
        if cap.len() != 6 {
            panic!("Invalid range");
        }

        fields.insert(
            (&cap[1].to_string()).clone(),
            Ranges {
                range1: Range {
                    start: (&cap[2]).parse().unwrap(),
                    end: (&cap[3]).parse::<usize>().unwrap() + 1,
                },
                range2: Range {
                    start: (&cap[4]).parse().unwrap(),
                    end: (&cap[5]).parse::<usize>().unwrap() + 1,
                },
            },
        );
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
    let nearby_tickets: Vec<Ticket> = nearby_tickets_str
        .trim()
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Note::new(fields, ticket, nearby_tickets)
}

#[test]
fn test_part1() {
    assert_eq!(71, part1(_get_data_test()));
    assert_eq!(21978, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(1053686852011, part2(get_data()));
}
