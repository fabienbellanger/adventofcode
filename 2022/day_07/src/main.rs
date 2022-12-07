use std::{fs, collections::{HashMap}};

#[derive(Debug)]
enum Output {
    Command(Command),
    Directory(String),
    File(String, usize),
}

#[derive(Debug)]
enum Command {
    CdRoot,
    CdParent,
    CdChild(String),
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

fn build_outputs(outputs: Vec<Output>) -> HashMap<String, (Vec<File>, usize)> {
    let mut result: HashMap<String, (Vec<File>, usize)> = HashMap::new();
    let mut current = String::new();

    for output in outputs {
        match output {
            Output::File(name, size) => {
                if let Some((r, t)) = result.get_mut(&current) {
                    r.push(File{name, size});
                    *t = *t + size;
                }
            },
            Output::Directory(_name) => (),
            Output::Command(cmd) => match cmd {
                Command::CdRoot => {
                    current = "/".to_owned();
                    result.insert(current.clone(), (vec![], 0));
                },
                Command::CdParent => {
                    if &current != "/" {
                        let mut parts = current.split_terminator('/').collect::<Vec<_>>();
                        parts.pop();
                        current = parts.join("/");
                        if current.is_empty() {
                            current = "/".to_owned();
                        }
                    }
                },
                Command::CdChild(dir) => {
                    current.push_str(&dir);
                    current.push_str("/");
                    result.insert(current.clone(), (vec![], 0));
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
    dbg!(&directories, &keys);

    0
}

// fn part2(data: Vec<Output>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(95437, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
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
                let (_, name) = line.split_once(' ').unwrap();
                Some(Output::Directory(name.to_string()))
            } else if !line.starts_with("$ ls"){
                let (size, name) = line.split_once(' ').unwrap();
                Some(Output::File(name.to_string(), size.parse::<usize>().unwrap_or_default()))
            } else {
                None
            }
        })
        .collect()
}
