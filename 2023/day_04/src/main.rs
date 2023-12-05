#![allow(unused_variables)]
use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug, Default, Clone)]
struct Card {
    copies: usize,
    winners: Vec<Number>,
    others: Vec<Number>,
}

type Number = u8;

impl Card {
    fn count_winners(&self) -> usize {
        self.winners
            .iter()
            .filter_map(|number| {
                if self.others.contains(number) {
                    Some(*number)
                } else {
                    None
                }
            })
            .count()
    }

    fn score_1(&self) -> usize {
        match self.count_winners() {
            0 => 0,
            n => 2_usize.pow(n as u32 - 1),
        }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Vec<Card>) -> usize {
    data.iter().map(|card| card.score_1()).sum()
}

fn part2(data: Vec<Card>) -> usize {
    let mut cards = data;

    for i in 0..cards.len() {
        let card = cards.get(i).unwrap();
        let count_winners = card.count_winners();
        let to_copy = card.copies;

        for next_card in cards.iter_mut().take(i + count_winners + 1).skip(i + 1) {
            next_card.copies += to_copy;
        }
    }

    cards.into_iter().map(|card| card.copies).sum()
}

fn parse_input(file: &str) -> Vec<Card> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_, line) = line.split_once(": ").unwrap();
            let (winning_part, other_part) = line.split_once(" | ").unwrap();

            let winners = winning_part
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<Number>().unwrap())
                .collect();

            let others = other_part
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<Number>().unwrap())
                .collect();

            Card {
                copies: 1,
                winners,
                others,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(parse_input(TEST)));
        assert_eq!(26_914, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2(parse_input(TEST)));
        assert_eq!(13_080_971, part2(parse_input(INPUT)));
    }
}
