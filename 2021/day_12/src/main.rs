use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn is_lower(s: &String) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn find_path(
    paths: &HashMap<String, Vec<String>>,
    current: String,
    visited: &mut HashSet<String>,
    path: &mut Vec<String>,
) -> usize {
    /*
    start,A,b,A,c,A,end
    start,A,b,A,end
    X start,A,b,end
    start,A,c,A,b,A,end
    start,A,c,A,b,end
    start,A,c,A,end
    start,A,end
    start,b,A,c,A,end
    start,b,A,end
    start,b,end
    */

    path.push(current.clone());

    if current == "end".to_owned() {
        println!("======> {:?}", &path);
        return 1;
    }

    let mut count = 0;
    for next in paths.get(&current).unwrap() {
        // println!("{} {:?}", next, visited);
        if is_lower(next) {
            if visited.contains(next) {
                continue;
            } else {
                visited.insert(next.clone());
            }
        }

        count += find_path(&paths, next.clone(), &mut visited.clone(), &mut path.clone());
    }

    count
}

fn part1(paths: HashMap<String, Vec<String>>) -> usize {
    dbg!(&paths);
    let mut visited = HashSet::new();
    visited.insert("start".to_string());
    find_path(&paths, "start".to_string(), &mut visited, &mut Vec::new())
}

fn part2() -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(10, part1(get_data("test.txt")));
    // assert_eq!(344297, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    // assert_eq!(168, part2(get_data("test.txt")));
    // assert_eq!(97164301, part2(get_data("input.txt")));
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
