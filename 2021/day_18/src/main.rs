use std::fs;

#[derive(Debug, PartialEq)]
enum Number {
    Regular(usize),
    Pair((Box<Number>, Box<Number>)),
}

impl Number {
    fn print(&self) -> String {
        match self {
            Self::Regular(d) => format!("{}", d),
            Self::Pair(p) => {
                format!("[{},{}]", p.0.print(), p.1.print(),)
            }
        }
    }

    fn parse(data: &mut Vec<char>) -> Self {
        match data.first().unwrap() {
            '[' => {
                // New pair
                *data = data[1..].to_vec(); // => '['
                let first_pair = Self::parse(data);
                *data = data[1..].to_vec(); // => ','
                let second_pair = Self::parse(data);
                *data = data[1..].to_vec(); // => ']'

                Number::Pair((Box::new(first_pair), Box::new(second_pair)))
            }
            n => {
                // Regular
                let n = Number::Regular(n.to_digit(10).unwrap() as usize);
                *data = data[1..].to_vec();

                n
            }
        }
    }

    fn add(self, number: Self) -> Self {
        Self::Pair((Box::new(self), Box::new(number)))
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
            String::from("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"),
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
    fn test_magnitude() {
        assert_eq!(
            143,
            Number::magnitude(&Number::parse(&mut String::from("[[1,2],[[3,4],5]]").chars().collect()))
        );
        assert_eq!(
            1384,
            Number::magnitude(&Number::parse(
                &mut String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").chars().collect()
            ))
        );
        assert_eq!(
            445,
            Number::magnitude(&Number::parse(
                &mut String::from("[[[[1,1],[2,2]],[3,3]],[4,4]]").chars().collect()
            ))
        );
        assert_eq!(
            791,
            Number::magnitude(&Number::parse(
                &mut String::from("[[[[3,0],[5,3]],[4,4]],[5,5]]").chars().collect()
            ))
        );
        assert_eq!(
            1137,
            Number::magnitude(&Number::parse(
                &mut String::from("[[[[5,0],[7,4]],[5,5]],[6,6]]").chars().collect()
            ))
        );
        assert_eq!(
            3488,
            Number::magnitude(&Number::parse(
                &mut String::from("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                    .chars()
                    .collect()
            ))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(4140, part1(get_data("test.txt")));
        // assert_eq!(344297, part1(get_data("input.txt")));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(168, part2(get_data("test.txt")));
        // assert_eq!(97164301, part2(get_data("input.txt")));
    }
}
