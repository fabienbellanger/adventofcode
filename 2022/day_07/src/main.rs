use std::fs;

#[derive(Debug)]
enum Output {
    Command(Command),
    Directory(String),
    File(usize, String),
}

#[derive(Debug)]
enum Command {
    CD(CD),
    LS,
}

#[derive(Debug)]
enum CD {
    Root,
    Parent,
    Child(String),
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Output>) -> usize {
    dbg!(&data);
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
        .map(|line| {
            if line.starts_with("$ ls") {
                Output::Command(Command::LS)
            } else if line.starts_with("$ cd ") {
                let (_, name) = line.split_once("$ cd ").unwrap();
                Output::Command(Command::CD(match name {
                    "/" => CD::Root,
                    ".." => CD::Parent,
                    _ => CD::Child(name.to_string())
                }))
            } else if line.starts_with("dir ") {
                let (_, name) = line.split_once(' ').unwrap();
                Output::Directory(name.to_string())
            } else {
                let (size, name) = line.split_once(' ').unwrap();
                Output::File(size.parse::<usize>().unwrap_or_default(), name.to_string())
            }
        })
        .collect()
}
