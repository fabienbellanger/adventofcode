use std::{collections::BTreeMap, fs};

#[derive(Debug)]
enum Output {
    Command(Command),
    File(String, usize),
}

#[derive(Debug)]
enum Command {
    CdRoot,
    CdParent,
    CdChild(String),
}

fn build_outputs(outputs: Vec<Output>) -> BTreeMap<String, usize> {
    let mut result: BTreeMap<String, usize> = BTreeMap::new();
    let mut current = String::new();

    for output in outputs {
        match output {
            Output::File(_name, size) => {
                if let Some(s) = result.get_mut(&current) {
                    *s = *s + size;
                }
            }
            Output::Command(cmd) => match cmd {
                Command::CdRoot => {
                    current = "/".to_owned();
                    result.insert(current.clone(), 0);
                }
                Command::CdParent => {
                    if &current != "/" {
                        let mut parts = current.split_terminator('/').collect::<Vec<_>>();
                        parts.pop();
                        current = parts.join("/");
                        current.push('/');
                    }
                }
                Command::CdChild(dir) => {
                    current.push_str(&dir);
                    current.push_str("/");
                    result.insert(current.clone(), 0);
                }
            },
        }
    }

    let keys = result.keys().collect::<Vec<_>>();
    let mut directories = BTreeMap::new();
    for (name, _size) in result.iter() {
        let mut sum = 0;
        for sd in keys.iter() {
            if sd.starts_with(name) {
                let size_sub = result.get(*sd).unwrap();
                sum += size_sub;
            }
        }
        directories.insert(name.clone(), sum);
    }

    directories
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Output>) -> usize {
    let directories = build_outputs(data);

    directories
        .iter()
        .map(|(_, size)| *size)
        .filter(|n| *n <= 100_000)
        .sum()
}

fn part2(data: Vec<Output>) -> usize {
    let directories = build_outputs(data);
    let max_size = directories.get("/").unwrap();
    let delta = 30_000_000 - (70_000_000 - max_size);

    directories
        .iter()
        .map(|(_, size)| *size)
        .filter(|s| *s >= delta)
        .min()
        .unwrap_or_default()
}

#[test]
fn test_part1() {
    assert_eq!(95_437, part1(get_data("test.txt")));
    assert_eq!(1_325_919, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(24_933_642, part2(get_data("test.txt")));
    assert_eq!(2_050_735, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Output> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .filter_map(|line| {
            if line.starts_with("$ cd ") {
                let (_, name) = line.split_once("$ cd ").unwrap();
                Some(Output::Command(match name {
                    "/" => Command::CdRoot,
                    ".." => Command::CdParent,
                    _ => Command::CdChild(name.to_string()),
                }))
            } else if line.starts_with("dir ") {
                None
            } else if !line.starts_with("$ ls") {
                let (size, name) = line.split_once(' ').unwrap();
                Some(Output::File(
                    name.to_string(),
                    size.parse::<usize>().unwrap_or_default(),
                ))
            } else {
                None
            }
        })
        .collect()
}
