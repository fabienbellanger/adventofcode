use std::{collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq)]
struct Rule {
    rules: Vec<Vec<usize>>,
    val: String,
}

impl Rule {
    fn init() -> Self {
        Self {
            rules: Vec::new(),
            val: String::from(""),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot find input.txt");

    println!("Part 1 result: {}", part1(get_data(&input)));
    // println!("Part 2 result: {}", part2(get_data(&input)));
}

fn part1(data: (HashMap<usize, Rule>, Vec<String>)) -> usize {
    let possible_messages = get_all_messages(&data.0, 0, &vec![]);

    let mut result = 0;
    let messages = data.1;

    for message in messages {
        if possible_messages.contains(&message) {
            result += 1;
        }
    }

    result
}

fn get_all_messages(
    rules: &HashMap<usize, Rule>,
    index: usize,
    previous_messages: &[String],
) -> Vec<String> {
    let mut messages = Vec::new();

    let rule = rules.get(&index).unwrap();

    if !rule.val.is_empty() {
        if previous_messages.is_empty() {
            messages.push(rule.val.clone());
        } else {
            messages = previous_messages
                .iter()
                .map(|message| {
                    let mut m = message.clone();
                    m.push_str(&rule.val);
                    m
                })
                .collect();
        }
    } else {
        let mut tmp = vec![];

        for part in rule.rules.iter() {
            let mut previous_merge = previous_messages.to_owned();

            for i in part {
                let result = get_all_messages(&rules, *i, &previous_merge);
                previous_merge = result;
            }

            tmp.append(&mut previous_merge);
        }

        messages = tmp;
    }

    messages
}

fn parse_rules_line(line: &str) -> (usize, Rule) {
    let mut rule = Rule::init();
    let mut parts = line.split(": ");
    let index = parts.next().unwrap().parse().unwrap();
    let parts = parts.next().unwrap();

    if parts.contains('a') || parts.contains('b') {
        rule.val = parts.chars().nth(1).unwrap().to_string();
    } else if parts.contains('|') {
        let parts: Vec<Vec<usize>> = parts
            .split('|')
            .map(|part| {
                let part: Vec<usize> = part
                    .split_whitespace()
                    .map(|part| part.parse().unwrap())
                    .collect();
                part
            })
            .collect();
        rule.rules = parts;
    } else {
        let parts: Vec<usize> = parts
            .split_whitespace()
            .map(|p| p.parse().unwrap())
            .collect();
        rule.rules.push(parts);
    }

    (index, rule)
}

fn get_data(data: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let mut parts = data.trim().split("\n\n");
    let rules_part = parts.next().unwrap();
    let messages = parts.next().unwrap();

    // Rules
    let mut rules = HashMap::new();
    for rule in rules_part.lines() {
        let (i, r) = parse_rules_line(rule);
        rules.insert(i, r);
    }

    // Messages
    let messages: Vec<String> = messages.lines().map(|l| l.to_owned()).collect();

    (rules, messages)
}

#[test]
fn test_get_all_messages() {
    let input = r#"
0: 1 1
1: 2 3
2: "a"
3: "b"

abab"#;
    let data = get_data(input);
    assert_eq!(vec!["abab"], get_all_messages(&data.0, 0, &vec![]));

    let input = r#"
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
    let data = get_data(input);
    assert_eq!(
        vec!["aaaabb", "abbabb", "aaabab", "abbbab", "aabaab", "abaaab", "aabbbb", "ababbb"],
        get_all_messages(&data.0, 0, &vec![])
    );
}

#[test]
fn test_part1() {
    let input = r#"
0: 1 1
1: 2 3
2: "a"
3: "b"

abab"#;
    assert_eq!(1, part1(get_data(input)));

    let input = r#"
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
    assert_eq!(2, part1(get_data(input)));

    let input = fs::read_to_string("input.txt").expect("Cannot find input.txt");
    assert_eq!(182, part1(get_data(&input)));
}

#[test]
fn test_parse_rules_line() {
    let (idx, rule) = parse_rules_line(r#"29: "b""#);
    assert_eq!(29, idx);
    assert_eq!(
        Rule {
            val: String::from("b"),
            rules: vec![],
        },
        rule
    );

    let (idx, rule) = parse_rules_line("0: 4 1 5");
    assert_eq!(0, idx);
    assert_eq!(
        Rule {
            val: String::from(""),
            rules: vec![vec![4, 1, 5]],
        },
        rule
    );

    let (idx, rule) = parse_rules_line("14: 29 1 | 106 52");
    assert_eq!(14, idx);
    assert_eq!(
        Rule {
            val: String::from(""),
            rules: vec![vec![29, 1], vec![106, 52]],
        },
        rule
    );
}
