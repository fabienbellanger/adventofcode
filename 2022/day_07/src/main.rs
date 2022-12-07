use std::{fs, collections::{HashMap, VecDeque}};

#[derive(Debug)]
enum Output {
    Command(Command),
    Directory(String),
    File(usize, String),
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

fn build_outputs(outputs: Vec<Output>) -> HashMap<String, Vec<File>> {
    dbg!(&outputs);

    let mut result = HashMap::new();

    let mut current_path = "";
    let mut current: Vec<String> = Vec::new();

    for output in outputs {
        match output {
            Output::File(name, size) => (),
            Output::Directory(name) => (),
            Output::Command(cmd) => match cmd {
                Command::CdRoot => {
                    current.clear();
                    current.push("/".to_string());
                    result.insert(current.join("+"), vec![]); // Use entry
                    ()
                },
                Command::CdParent => {
                    current.pop();
                    ()
                },
                Command::CdChild(dir) => {
                    // current.push_back(dir.clone());
                    // // current_path.push_str(&dir);
                    // // current_path.push('/');
                    // let mut tmp = String::from(current_path);
                    // tmp.push_str(&dir);
                    // tmp.push('/');
                    // current_path = tmp.as_str();
                    // result.insert(current_path.to_string(), vec![]);
                    current.push(dir);
                    result.insert(current.join("+"), vec![]); // Use entry
                    ()
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
    dbg!(build_outputs(data));
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
                Some(Output::File(size.parse::<usize>().unwrap_or_default(), name.to_string()))
            } else {
                None
            }
        })
        .collect()
}
