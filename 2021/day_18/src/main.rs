use std::{cmp::Ordering, fs};

#[derive(Debug, PartialEq)]
enum Number {
    Regular(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn print(&self) -> String {
        match self {
            Self::Regular(d) => format!("{}", d),
            Self::Pair(a, b) => {
                format!("[{},{}]", a.print(), b.print())
            }
        }
    }

    fn parse(data: &mut Vec<char>) -> Self {
        match data.first().unwrap() {
            '[' => {
                // New pair
                *data = data[1..].to_vec(); // => '['
                let first = Self::parse(data);
                *data = data[1..].to_vec(); // => ','
                let second = Self::parse(data);
                *data = data[1..].to_vec(); // => ']'

                Self::Pair(Box::new(first), Box::new(second))
            }
            n => {
                // Regular
                let n = Self::Regular(n.to_digit(10).unwrap() as usize);
                *data = data[1..].to_vec();

                n
            }
        }
    }

    fn get_regular(self) -> usize {
        match self {
            Self::Regular(v) => v,
            _ => panic!("not regular number"),
        }
    }

    fn add_to_left(&mut self, value: usize) {
        match self {
            Number::Pair(_left, right) => {
                right.add_to_left(value);
            }
            Number::Regular(r) => {
                *r += value;
            }
        }
    }

    fn add_to_right(&mut self, value: usize) {
        match self {
            Number::Pair(left, _right) => {
                left.add_to_right(value);
            }
            Number::Regular(r) => {
                *r += value;
            }
        }
    }

    fn add(self, number: Self) -> Self {
        Self::Pair(Box::new(self), Box::new(number))
    }

    fn split(self, stop: &mut bool) -> Self {
        match self {
            Self::Regular(d) => {
                if d > 9 {
                    let middle = d / 2;
                    let first = middle;
                    let second = middle + d % 2;

                    *stop = true;

                    Self::Pair(Box::new(Self::Regular(first)), Box::new(Self::Regular(second)))
                } else {
                    self
                }
            }
            Self::Pair(a, b) => {
                if *stop {
                    return Self::Pair(a, b);
                }

                let a = a.split(stop);

                if *stop {
                    Self::Pair(Box::new(a), b)
                } else {
                    Self::Pair(Box::new(a), Box::new(b.split(stop)))
                }
            }
        }
    }

    fn explode(self, depth: usize, exploded: &mut bool) -> (Option<usize>, Number, Option<usize>) {
        match self {
            Self::Pair(left, mut right) => {
                match depth.cmp(&3) {
                    Ordering::Greater => (None, Self::Pair(left, right), None),
                    Ordering::Less => {
                        // Explode left first
                        let (left_val, mut left, mut right_val) = left.explode(depth + 1, exploded);
                        if let Some(right_val) = right_val {
                            right.add_to_right(right_val);
                        }

                        if *exploded {
                            (left_val, Self::Pair(Box::new(left), right), right_val)
                        } else {
                            // Explode right if left not explode
                            let (left_val, right, right_val) = right.explode(depth + 1, exploded);
                            if let Some(left_val) = left_val {
                                left.add_to_right(left_val);
                            }

                            (left_val, Self::Pair(Box::new(left), Box::new(right)), right_val)
                        }
                    }
                    Ordering::Equal => {
                        match (*left, *right) {
                            (Self::Pair(left, right), mut value) => {
                                *exploded = true;

                                // left and right must be regular number
                                let left = left.get_regular();
                                let right = right.get_regular();
                                value.add_to_right(right);

                                (
                                    Some(left),
                                    Self::Pair(Box::new(Self::Regular(0)), Box::new(value)),
                                    None,
                                )
                            }
                            (mut value, Self::Pair(left, right)) => {
                                *exploded = true;

                                // left and right must be regular number
                                let left = left.get_regular();
                                let right = right.get_regular();
                                value.add_to_left(left);

                                (
                                    None,
                                    Self::Pair(Box::new(value), Box::new(Self::Regular(0))),
                                    Some(right),
                                )
                            }
                            (left, right) => (None, Self::Pair(Box::new(left), Box::new(right)), None),
                        }
                    }
                }
            }
            Self::Regular(_) => (None, self, None),
        }
    }

    fn reduce(self) -> Self {
        todo!();
        self
    }

    fn magnitude(&self) -> usize {
        0
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(_numbers: Vec<Number>) -> usize {
    0
}

fn part2(_numbers: Vec<Number>) -> usize {
    0
}

fn get_data(file: &str) -> Vec<Number> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|line| Number::parse(&mut line.to_string().chars().collect()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_parse_number() {
        assert_eq!(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            Number::print(&Number::parse(
                &mut String::from("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
                    .chars()
                    .collect()
            ))
        );
    }

    #[test]
    fn test_add_numbers() {
        let n1 = Number::parse(&mut String::from("[1,2]").chars().collect());
        let n2 = Number::parse(&mut String::from("[[3,4],5]").chars().collect());

        assert_eq!(String::from("[[1,2],[[3,4],5]]"), Number::print(&n1.add(n2)));
    }

    #[test]
    fn test_split() {
        let n1 = Number::Regular(15);
        let expected = Number::Pair(Box::new(Number::Regular(7)), Box::new(Number::Regular(8)));
        assert_eq!(n1.split(&mut true), expected);
    }

    #[test]
    fn test_explode() {
        let number = Number::parse(&mut String::from("[[[[[9,8],1],2],3],4]").chars().collect());
        assert_eq!("[[[[0,9],2],3],4]", Number::print(&number.explode(0, &mut false).1));

        let number = Number::parse(&mut String::from("[7,[6,[5,[4,[3,2]]]]]").chars().collect());
        assert_eq!("[7,[6,[5,[7,0]]]]", Number::print(&number.explode(0, &mut false).1));

        let number = Number::parse(&mut String::from("[[6,[5,[4,[3,2]]]],1]").chars().collect());
        assert_eq!("[[6,[5,[7,0]]],3]", Number::print(&number.explode(0, &mut false).1));

        // TODO: Dérouler l'algo
        let number = Number::parse(&mut String::from("[[6,[5,[4,[3,2]]]],[1,2]]").chars().collect());
        assert_eq!("[[6,[5,[7,0]]],[3,2]]", Number::print(&number.explode(0, &mut false).1));

        let number = Number::parse(&mut String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").chars().collect());
        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            Number::print(&number.explode(0, &mut false).1)
        );

        let number = Number::parse(&mut String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").chars().collect());
        assert_eq!(
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            Number::print(&number.explode(0, &mut false).1)
        );

        let number = Number::parse(&mut String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").chars().collect());
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            Number::print(&number.explode(0, &mut false).1)
        );
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            Number::print(&Number::parse(
                &mut String::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").chars().collect()
            ))
        );
    }

    #[test]
    fn test_magnitude() {
        // assert_eq!(
        //     143,
        //     Number::magnitude(&Number::parse(&mut String::from("[[1,2],[[3,4],5]]").chars().collect()))
        // );
        // assert_eq!(
        //     1384,
        //     Number::magnitude(&Number::parse(
        //         &mut String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").chars().collect()
        //     ))
        // );
        // assert_eq!(
        //     445,
        //     Number::magnitude(&Number::parse(
        //         &mut String::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").chars().collect()
        //     ))
        // );
        // assert_eq!(
        //     791,
        //     Number::magnitude(&Number::parse(
        //         &mut String::from("[[[[3,0],[5,3]],[4,4]],[5,5]]").chars().collect()
        //     ))
        // );
        // assert_eq!(
        //     1137,
        //     Number::magnitude(&Number::parse(
        //         &mut String::from("[[[[5,0],[7,4]],[5,5]],[6,6]]").chars().collect()
        //     ))
        // );
        // assert_eq!(
        //     3488,
        //     Number::magnitude(&Number::parse(
        //         &mut String::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        //             .chars()
        //             .collect()
        //     ))
        // );
    }

    #[test]
    fn test_part1() {
        // assert_eq!(4140, part1(get_data("test.txt")));
        // assert_eq!(344297, part1(get_data("input.txt")));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(168, part2(get_data("test.txt")));
        // assert_eq!(97164301, part2(get_data("input.txt")));
    }
}
