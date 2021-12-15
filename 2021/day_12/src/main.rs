use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn is_small_cave(s: &str) -> bool {
    s.chars().next().unwrap().is_lowercase()
}

fn find_path(
    paths: &HashMap<String, Vec<String>>,
    current: String,
    visited: HashSet<String>,
    path: &mut Vec<String>,
    twice: bool,
) -> usize {
    path.push(current.clone());

    if current == *"end" {
        // println!("===> {:?}", &path);
        return 1;
    }

    let mut count = 0;
    for next in paths.get(&current).unwrap() {
        let mut visited = visited.clone();
        let mut twice = twice;
        if is_small_cave(next) {
            if visited.contains(next) {
                if !twice || next == "start" || next == "end" {
                    continue;
                } else {
                    twice = false;
                }
            } else {
                visited.insert(next.clone());
            }
        }

        count += find_path(paths, next.clone(), visited, &mut path.clone(), twice);
    }

    count
}

fn part1(paths: HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashSet::new();
    visited.insert("start".to_string());

    find_path(&paths, "start".to_string(), visited, &mut Vec::new(), false)
}

fn part2(paths: HashMap<String, Vec<String>>) -> usize {
    let mut visited = HashSet::new();
    visited.insert("start".to_string());

    find_path(&paths, "start".to_string(), visited, &mut Vec::new(), true)
}

#[test]
fn test_part1() {
    assert_eq!(10, part1(get_data("test.txt")));
    assert_eq!(4912, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(36, part2(get_data("test.txt")));
    assert_eq!(150004, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> HashMap<String, Vec<String>> {
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .for_each(|line| {
            let (from, to) = line.split_once('-').unwrap();

            paths.entry(from.to_string()).or_default().push(to.to_string());
            paths.entry(to.to_string()).or_default().push(from.to_string());
        });

    paths
}
