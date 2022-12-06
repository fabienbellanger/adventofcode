use std::fs;

#[derive(Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

type Stack = Vec<char>;

#[derive(Debug)]
struct Expedition {
    moves: Vec<Move>,
    stacks: Vec<Stack>,
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Expedition) -> String {
    let moves = data.moves;
    let mut stacks = data.stacks;
    
    let mut result = String::new();
    for m in moves {
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain
    }

    result
}

// fn part2(data: Expedition) -> String {
//     "".to_owned()
// }

#[test]
fn test_part1() {
    assert_eq!(String::from("CMZ"), part1(get_data("test.txt")));
    // assert_eq!(String::from(""), part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Expedition {
    let content = fs::read_to_string(file)
        .expect("Cannot read the file input.txt");

    let mut parts = content.split("\n\n");
    let stacks_parts = parts.next().unwrap();
    let moves_parts = parts.next().unwrap();

    // Stacks
    // ------
    let stacks_number = stacks_parts.split('\n').rev().take(1)
        .map(|l| l.trim().chars()
            .map(|c| c.to_digit(10).unwrap_or_default())
            .max()
            .unwrap_or_default())
        .max()
        .unwrap_or_default() as usize;
    
    let mut stacks: Vec<Stack> = vec![vec![];stacks_number]; 
    stacks_parts.split('\n').rev().skip(1)
        .for_each(|line| {
            let list: Vec<char> = line.chars().collect();
            for i in 0..stacks_number {
                let j: usize = i * (stacks_number + 1) + 1;
                if let Some(c) = list.get(j) {
                    if *c != ' ' {
                        stacks[i].push(*c);
                    }
                }
            }
        });

    // Moves
    // -----
    let moves = moves_parts.trim()
        .lines()
        .map(|line| {
            let mut line_parts = line.split_whitespace();

            line_parts.next();
            let n = line_parts.next().unwrap().parse::<usize>().unwrap();
            
            line_parts.next();
            let from = line_parts.next().unwrap().parse::<usize>().unwrap() - 1;
            
            line_parts.next();
            let to = line_parts.next().unwrap().parse::<usize>().unwrap() - 1;

            Move{n, from, to}
        })
        .collect();

    Expedition{moves, stacks}
}
