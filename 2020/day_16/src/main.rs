use regex::Regex;
use std::{collections::HashSet, fs, ops::Range};

type Ticket = Vec<usize>;

#[derive(Debug, Clone, PartialEq)]
struct Field {
    name: String,
    range1: Range<usize>,
    range2: Range<usize>,
}

#[derive(Debug)]
struct Note {
    fields: Vec<Field>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Note {
    fn new(fields: Vec<Field>, ticket: Ticket, nearby_tickets: Vec<Ticket>) -> Self {
        Self {
            fields,
            ticket,
            nearby_tickets,
        }
    }

    fn is_ticket_values_in_fields(&self, value: usize) -> bool {
        for field in &self.fields {
            if field.range1.contains(&value) || field.range2.contains(&value) {
                return true;
            }
        }
        false
    }

    fn get_right_fields(&self, columun: Vec<usize>) -> Vec<Field> {
        let mut correct = self.fields.clone();
        for value in &columun {
            let mut i = 0usize;
            for field in &self.fields {
                if correct.get(i).is_some() && !field.range1.contains(&value) && !field.range2.contains(&value) {
                    correct.remove(i);
                }
                i += 1;
            }
        }
        correct
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
    let mut combinaisons: Vec<Vec<Field>> = Vec::new();
    for col in 0..note.fields.len() {
        let cols = note.nearby_tickets
            .iter()
            .map(|ticket| {
                ticket[col]
            })
            .collect();
        combinaisons.push(note.get_right_fields(cols));
    }
    dbg!(&combinaisons);

    let mut best_combinaison: Vec<Field> = Vec::new();
    for combinaison in &combinaisons {
        for choice in combinaison {
            if best_combinaison.contains(&choice) {
                continue;
            } else {
                best_combinaison.push((*choice).clone());
                break;
            }
        }
    }

    dbg!(&best_combinaison);

    let mut result = 1;
    for (index, combinaison) in best_combinaison.iter().enumerate() {
        if combinaison.name.contains("departure") {
            dbg!(note.ticket[index]);
            result *= note.ticket[index];
        }
    }
    
    result
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

    // Ranges
    let ranges_regex = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let mut fields: Vec<Field> = Vec::new();
    for line in ranges.trim().lines() {
        if !ranges_regex.is_match(line) {
            panic!("Invalid range");
        }

        let cap = ranges_regex.captures(line).unwrap();
        if cap.len() != 6 {
            panic!("Invalid range");
        }

        fields.push(Field {
            name: (&cap[1].to_string()).clone(),
            range1: Range { 
                start: (&cap[2]).parse().unwrap(), 
                end: (&cap[3]).parse::<usize>().unwrap() + 1,
            },
            range2: Range { 
                start: (&cap[4]).parse().unwrap(), 
                end: (&cap[5]).parse::<usize>().unwrap() + 1,
            },
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
    assert_eq!(208, part2(get_data()));
    // assert_eq!(208, part2(_get_data_test2()));
    // assert_eq!(3348493585827, part2());
}
