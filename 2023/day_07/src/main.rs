#![allow(unused_variables)]
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::{fmt, fs};

const INPUT: &str = "input.txt";

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum HandType {
    FiveKind(u8),
    FourKind(u8),
    Full(u8, u8),
    ThreeKind(u8),
    TwoPair(u8, u8),
    OnePair(u8),
    #[default]
    High,
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FiveKind(c) => write!(f, "Five of kind ({c})"),
            Self::FourKind(c) => write!(f, "Four of kind ({c})"),
            Self::Full(c1, c2) => write!(f, "Five of kind ({c1}, {c2})"),
            Self::ThreeKind(c) => write!(f, "Three of kind ({c})"),
            Self::TwoPair(c1, c2) => write!(f, "Two pair ({c1}, {c2})"),
            Self::OnePair(c) => write!(f, "One pair ({c})  "),
            Self::High => write!(f, "High           "),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Hand {
    bid: usize,
    hand_type: HandType,
    others: Vec<u8>,
    cards: Vec<u8>,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "bid={:<6} | type={:<24}\t| others={:?} | cards={:?}",
            self.bid, self.hand_type, self.others, self.cards
        )
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.hand_type.clone(), self.cards.clone()) {
            // FiveKind
            (HandType::FiveKind(_), o1) => match (other.hand_type.clone(), other.cards.clone()) {
                (HandType::FiveKind(_), o2) => Some(o1.cmp(&o2)),
                _ => Some(Ordering::Greater),
            },

            // FourKind
            (HandType::FourKind(c), o1) => match (other.hand_type.clone(), other.cards.clone()) {
                (HandType::FiveKind(_), _) => Some(Ordering::Less),
                (HandType::FourKind(d), o2) => Some(o1.cmp(&o2)),
                _ => Some(Ordering::Greater),
            },

            // Full
            (HandType::Full(c1, c2), o1) => match (other.hand_type.clone(), other.cards.clone()) {
                (HandType::FiveKind(_), _) | (HandType::FourKind(_), _) => Some(Ordering::Less),
                (HandType::Full(d1, d2), o2) => Some(o1.cmp(&o2)),
                _ => Some(Ordering::Greater),
            },

            // ThreeKind
            (HandType::ThreeKind(c), o1) => match (other.hand_type.clone(), other.cards.clone()) {
                (HandType::FiveKind(_), _) | (HandType::FourKind(_), _) | (HandType::Full(_, _), _) => {
                    Some(Ordering::Less)
                }
                (HandType::ThreeKind(d), o2) => Some(o1.cmp(&o2)),
                _ => Some(Ordering::Greater),
            },

            // TwoPair
            (HandType::TwoPair(c1, c2), o1) => match (other.hand_type.clone(), other.cards.clone()) {
                (HandType::FiveKind(_), _)
                | (HandType::FourKind(_), _)
                | (HandType::Full(_, _), _)
                | (HandType::ThreeKind(_), _) => Some(Ordering::Less),
                (HandType::TwoPair(d1, d2), o2) => Some(o1.cmp(&o2)),
                _ => Some(Ordering::Greater),
            },

            // OnePair
            (HandType::OnePair(c), o1) => match (other.hand_type.clone(), other.cards.clone()) {
                (HandType::FiveKind(_), _)
                | (HandType::FourKind(_), _)
                | (HandType::Full(_, _), _)
                | (HandType::ThreeKind(_), _)
                | (HandType::TwoPair(_, _), _) => Some(Ordering::Less),
                (HandType::OnePair(d), o2) => Some(o1.cmp(&o2)),
                _ => Some(Ordering::Greater),
            },

            (HandType::High, o1) => match (other.hand_type.clone(), other.cards.clone()) {
                (HandType::High, o2) => Some(o1.cmp(&o2)),
                _ => Some(Ordering::Less),
            },
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

// 251605897 => KO
// 248083579 => KO
// 248075265 => KO
// 251558185 => KO
// 248774993 => KO
// 250638685 => ?
// 248141590 => ?
fn part1(data: Vec<Hand>) -> usize {
    let mut data = data;
    data.sort();

    for hand in data.iter() {
        println!("{hand}");
    }

    data.into_iter().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum()
}

fn part2(data: Vec<Hand>) -> usize {
    todo!()
}

fn search_counter(cards: &BTreeMap<u8, u8>, search: u8) -> Vec<u8> {
    let mut list = vec![];

    for (card, counter) in cards {
        if *counter == search {
            list.push(*card);
        }
    }

    list.sort_unstable();
    list.reverse();

    list
}

fn cards_to_hand_type(data: BTreeMap<u8, u8>) -> HandType {
    if data.len() == 1 {
        // FiveKind
        let cards = search_counter(&data, 5);

        return HandType::FiveKind(cards[0]);
    } else if data.len() == 2 {
        let cards = search_counter(&data, 4);
        return if !cards.is_empty() {
            // FourKind
            HandType::FourKind(cards[0])
        } else {
            // Full
            let cards_3 = search_counter(&data, 3);
            let cards_2 = search_counter(&data, 2);

            HandType::Full(cards_3[0], cards_2[0])
        };
    } else if data.len() == 3 {
        // ThreeKind or TwoPair
        let cards = search_counter(&data, 3);
        return if !cards.is_empty() {
            // ThreeKind
            HandType::ThreeKind(cards[0])
        } else {
            // TwoPair
            let cards_2 = search_counter(&data, 2);

            HandType::TwoPair(cards_2[0], cards_2[1])
        };
    } else if data.len() == 4 {
        // OnePair
        let cards = search_counter(&data, 2);

        return HandType::OnePair(cards[0]);
    }
    HandType::High
}

fn parse_input(file: &str) -> Vec<Hand> {
    fs::read_to_string(file)
        .expect(&format!("Cannot read the file {file}"))
        .lines()
        .map(|line| {
            let (cards, bid) = line.trim().split_once(' ').unwrap();

            let mut hand = BTreeMap::new();
            let mut all_cards = vec![];
            for card in cards.chars() {
                let value = match card.to_digit(10) {
                    Some(d) => d as u8,
                    None => match card {
                        'A' => 1,
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        _ => panic!("invalid card"),
                    },
                };

                all_cards.push(value);

                let entry = hand.entry(value).or_insert(0);
                *entry += 1;
            }

            // Others: cards with counter equal to 1
            let mut others = hand
                .iter()
                .filter(|(&card, &counter)| counter == 1)
                .map(|(&card, &counter)| card)
                .collect::<Vec<_>>();
            others.sort_unstable();
            others.reverse();

            Hand {
                bid: bid.parse().unwrap_or_default(),
                hand_type: cards_to_hand_type(hand),
                cards: all_cards,
                others,
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(6_440, part1(parse_input(TEST)));
        assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
