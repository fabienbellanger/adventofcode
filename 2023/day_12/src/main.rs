use std::fs;

const INPUT: &str = "input.txt";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Symbol {
    OK,
    Unknown,
    KO,
}

impl Symbol {
    fn is_ok(&self) -> bool {
        self != &Self::OK
    }
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::KO,
            '?' => Self::Unknown,
            _ => Self::OK,
        }
    }
}

#[derive(Debug, Clone)]
struct Record {
    conditions: Vec<usize>,
    data: Vec<Symbol>,
}

impl Record {
    fn is_pending(&self) -> bool {
        self.data.iter().any(|c| *c == Symbol::Unknown)
    }

    fn is_invalid(&self) -> bool {
        !self.is_pending() && !self.is_valid()
    }

    fn is_valid(&self) -> bool {
        if self.is_pending() {
            return false;
        }

        let condition_ko_count = self.conditions.iter().sum();
        let ko_count = self.data.iter().filter(|&c| *c == Symbol::KO).count();
        if ko_count != condition_ko_count {
            return false;
        }

        let data_len = self.data.len();
        let mut data = self.data.clone();
        for n in self.conditions.iter() {
            let pos = data.iter().position(|&s| s == Symbol::KO);

            if let Some(pos) = pos {
                if pos + n >= data_len {
                    return false;
                }

                let ko_list = &data[pos..pos + n];
                if !ko_list.iter().all(|&c| c == Symbol::KO) {
                    return false;
                }

                data = data.split_off(pos + n);
            } else {
                return false;
            }
        }

        true
    }

    fn next_not_ok(data: &Vec<Symbol>) -> Option<usize> {
        for (i, s) in data.iter().enumerate() {
            if s.is_ok() {
                return Some(i);
            }
        }

        None
    }

    // ????.??.???#??#??? 1,1,1,1,2,2
    fn arrangements(&self) -> Vec<Vec<Symbol>> {
        let mut cases = vec![self.data.clone()];
        let last_cond = self.conditions.len() - 1;
        let data_len = self.data.len();

        println!("   data: {:?}", self.data);

        for (i, n) in self.conditions.iter().enumerate() {
            println!("-> {i} : {n}");
            let mut new_cases = vec![];

            for case in cases.into_iter() {
                let mut current = Self::next_not_ok(&case).unwrap_or(0);

                while !(current == data_len || case[current] == Symbol::OK) {
                    println!("   current: {current} - {:?}", case[current]);

                    if current + n > data_len {
                        break;
                    }

                    // let mut new_case = case;

                    if i == last_cond {
                        // Last condition
                    } else {
                        // Other condition
                    }

                    current += 1;
                }
            }

            cases = new_cases;
        }

        dbg!(&cases);

        // TODO: Filter cases which have only # and .

        cases
    }
}

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn part1(data: Vec<Record>) -> usize {
    data.iter().map(|record| record.arrangements().len()).sum()
}

fn part2(data: Vec<Record>) -> usize {
    todo!()
}

fn parse_input(file: &str) -> Vec<Record> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {file}"))
        .trim()
        .lines()
        .map(|line| {
            let (data, conditions) = line.split_once(' ').unwrap();

            let conditions = conditions.split(',').map(|p| p.parse::<usize>().unwrap()).collect();
            let data = data.chars().map(|c| c.into()).collect();

            Record { conditions, data }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_record_is_pending() {
        let mut record = Record {
            conditions: vec![1, 1],
            data: vec![Symbol::OK, Symbol::KO, Symbol::OK, Symbol::Unknown, Symbol::KO],
        };
        assert_eq!(record.is_pending(), true);

        record.data = vec![Symbol::OK, Symbol::KO, Symbol::OK, Symbol::OK, Symbol::KO];
        assert_eq!(record.is_pending(), false);
    }

    #[test]
    fn test_record_is_valid() {
        let mut record = Record {
            conditions: vec![1, 1],
            data: vec![Symbol::OK, Symbol::KO, Symbol::OK, Symbol::Unknown, Symbol::KO],
        };
        assert_eq!(record.is_valid(), false);

        record.data = vec![Symbol::OK, Symbol::KO, Symbol::KO, Symbol::OK, Symbol::KO];
        assert_eq!(record.is_valid(), false);

        record.data = vec![Symbol::OK, Symbol::OK, Symbol::OK, Symbol::OK, Symbol::KO];
        assert_eq!(record.is_valid(), false);

        record.data = vec![Symbol::OK, Symbol::KO, Symbol::OK, Symbol::OK, Symbol::KO];
        assert_eq!(record.is_valid(), true);

        record.conditions = vec![2, 2];
        record.data = vec![Symbol::KO, Symbol::KO, Symbol::OK, Symbol::KO, Symbol::KO];
        assert_eq!(record.is_valid(), true);

        record.data = vec![Symbol::KO, Symbol::KO, Symbol::OK, Symbol::OK, Symbol::KO];
        assert_eq!(record.is_valid(), false);

        record.data = vec![Symbol::KO, Symbol::KO, Symbol::KO, Symbol::OK, Symbol::OK];
        assert_eq!(record.is_valid(), false);
    }

    #[test]
    fn test_record_is_invalid() {
        let mut record = Record {
            conditions: vec![1, 1],
            data: vec![Symbol::OK, Symbol::KO, Symbol::OK, Symbol::Unknown, Symbol::KO],
        };
        assert_eq!(record.is_invalid(), false);

        record.data = vec![Symbol::OK, Symbol::KO, Symbol::KO, Symbol::OK, Symbol::KO];
        assert_eq!(record.is_invalid(), true);

        record.data = vec![Symbol::OK, Symbol::OK, Symbol::OK, Symbol::OK, Symbol::KO];
        assert_eq!(record.is_invalid(), true);

        record.data = vec![Symbol::OK, Symbol::KO, Symbol::OK, Symbol::OK, Symbol::KO];
        assert_eq!(record.is_invalid(), false);
    }

    #[test]
    fn test_record_arrangement() {
        let mut record = Record {
            conditions: vec![1, 1],
            data: vec![Symbol::OK, Symbol::KO, Symbol::OK, Symbol::Unknown, Symbol::KO],
        };
        assert_eq!(
            record.arrangements(),
            vec![vec![Symbol::OK, Symbol::KO, Symbol::OK, Symbol::OK, Symbol::KO]]
        );

        record.conditions = vec![1, 1, 3];
        record.data = vec![
            Symbol::Unknown,
            Symbol::Unknown,
            Symbol::Unknown,
            Symbol::OK,
            Symbol::KO,
            Symbol::KO,
            Symbol::KO,
        ];
        assert_eq!(
            record.arrangements(),
            vec![vec![
                Symbol::KO,
                Symbol::OK,
                Symbol::KO,
                Symbol::OK,
                Symbol::KO,
                Symbol::KO,
                Symbol::KO,
            ]]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(parse_input(TEST)));
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
