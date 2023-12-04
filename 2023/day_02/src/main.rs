#![allow(unused_variables)]
use std::collections::HashMap;
use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

type Set = HashMap<Color, usize>;

#[derive(Debug, Clone)]
struct Game {
    number: usize,
    sets: Vec<Set>,
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Vec<Game>) -> usize {
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    data.into_iter()
        .filter_map(|game| {
            if game.sets.clone().iter().any(|set| {
                if let Some(red) = set.get(&Color::Red) {
                    if *red > max_red {
                        return true;
                    }
                }
                if let Some(green) = set.get(&Color::Green) {
                    if *green > max_green {
                        return true;
                    }
                }
                if let Some(blue) = set.get(&Color::Blue) {
                    if *blue > max_blue {
                        return true;
                    }
                }
                false
            }) {
                return None;
            }
            Some(game.number)
        })
        .sum()
}

fn part2(data: Vec<Game>) -> usize {
    data.into_iter()
        .map(|game| {
            let mut max_set = Set::from([(Color::Red, 1), (Color::Green, 1), (Color::Blue, 1)]);

            for set in game.sets.into_iter() {
                if set.get(&Color::Red).unwrap_or(&1) > &max_set[&Color::Red] {
                    max_set.insert(Color::Red, *set.get(&Color::Red).unwrap_or(&0));
                }
                if set.get(&Color::Green).unwrap_or(&1) > &max_set[&Color::Green] {
                    max_set.insert(Color::Green, *set.get(&Color::Green).unwrap_or(&0));
                }
                if set.get(&Color::Blue).unwrap_or(&1) > &max_set[&Color::Blue] {
                    max_set.insert(Color::Blue, *set.get(&Color::Blue).unwrap_or(&0));
                }
            }

            max_set[&Color::Red] * max_set[&Color::Green] * max_set[&Color::Blue]
        })
        .sum()
}

fn parse_input(file: &str) -> Vec<Game> {
    let mut games = vec![];

    let input = fs::read_to_string(file).expect("Cannot read the file");

    for line in input.trim().lines() {
        let (num_part, sets_part) = line.split_once(": ").unwrap();
        let number = num_part.strip_prefix("Game ").unwrap().parse::<usize>().unwrap();

        let sets = sets_part
            .split("; ")
            .map(|s| {
                s.split(", ")
                    .map(|c| {
                        let (n, color) = c.split_once(' ').unwrap();
                        let n = n.parse::<usize>().unwrap();
                        (color.try_into().unwrap(), n)
                    })
                    .collect::<Set>()
            })
            .collect::<Vec<_>>();

        games.push(Game { number, sets });
    }

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1: &str = "test1.txt";
    const TEST_2: &str = "test2.txt";

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(parse_input(TEST_1)));
        assert_eq!(2486, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2286, part2(parse_input(TEST_2)));
        assert_eq!(87984, part2(parse_input(INPUT)));
    }
}
