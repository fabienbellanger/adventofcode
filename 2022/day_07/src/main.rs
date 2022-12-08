/*use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", navigate(input));
}

fn cd(i: &str, cur:&mut Vec<String>) {
    match i {
        ".." => { cur.pop(); },
        "/" => { cur.clear(); },
        _ => {   cur.push(i.to_string()); }
    };
}

fn dir_size(from: &str, dirs: &HashMap<String, HashSet<(&str, usize)>>, counted: &mut HashSet<String>) -> usize {
    let mut s = dirs.get(from).unwrap().iter().filter_map(|&(f, size)| {
        let full_path = [from, f].join("");
        let n = (!counted.contains(&full_path)).then(|| size);
        counted.insert(full_path);
        n
    }).sum();

    dirs.keys()
        .filter(|l| l.starts_with(from))
        .filter(|&p| p != from)
        .for_each(|d| s += dir_size(d, dirs, counted));
    s
}


fn navigate(s: &str) -> (usize, usize) {
    let mut dir = vec![];
    let mut dirs: HashMap<String, HashSet<(&str, usize)>> = HashMap::new();

    for line in s.lines() {
        if line.starts_with("$") {
            let splits: Vec<_> = line.split_whitespace().collect();
            match splits[1] {
                "ls" => (),
                "cd" => cd(splits[2], &mut dir),
                _ => panic!()
            };
        } else {
            let current_dir = dir.join("/");
            if !dirs.contains_key(&current_dir) {
                dirs.insert(current_dir.clone(), HashSet::new());
            }

            if !line.starts_with("dir") {
                let (fsize, fname) =  line.split_once(' ').unwrap();
                let fsize = fsize.parse().unwrap();
                dirs.get_mut(&current_dir).unwrap().insert((fname, fsize));
            }
        }
    }

    dbg!(&dirs);
    
    let sizes: Vec<_> = dirs.keys()
        .map(|k| dir_size(k, &dirs, &mut HashSet::new()) )
        .collect();

    let delta = 30_000_000 + sizes.iter().max().unwrap() - 70_000_000;
    (sizes.iter().filter(|&&n| n <= 100_000).sum(),
        *sizes.iter().filter(|&&n| n >= delta).min().unwrap())
}*/

use std::{fs, collections::{BTreeMap}};

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
            },
            Output::Command(cmd) => match cmd {
                Command::CdRoot => {
                    current = "/".to_owned();
                    result.insert(current.clone(), 0);
                },
                Command::CdParent => {
                    if &current != "/" {
                        let mut parts = current.split_terminator('/').collect::<Vec<_>>();
                        parts.pop();
                        current = parts.join("/");
                        current.push('/');
                    }
                },
                Command::CdChild(dir) => {
                    current.push_str(&dir);
                    current.push_str("/");
                    result.insert(current.clone(), 0);
                },
            }
        }
    }

    result
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Output>) -> usize {
    let directories = build_outputs(data);
    let keys = directories.keys().collect::<Vec<_>>();

    let mut result = 0;
    for (name, _size) in directories.iter() {
        let mut sum = 0;

        // Find sub directories
        for sd in keys.iter() {
            if sd.starts_with(name) {
                let size_sub = directories.get(*sd).unwrap();
                sum += size_sub;
            }
        }

        if sum <= 100_000 {
            result += sum;
        }
    }

    result
}

// fn part2(data: Vec<Output>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(95437, part1(get_data("test.txt")));
    assert_eq!(1325919, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

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
                    _ => Command::CdChild(name.to_string())
                }))
            } else if line.starts_with("dir ") {
                None
            } else if !line.starts_with("$ ls"){
                let (size, name) = line.split_once(' ').unwrap();
                Some(Output::File(name.to_string(), size.parse::<usize>().unwrap_or_default()))
            } else {
                None
            }
        })
        .collect()
}
