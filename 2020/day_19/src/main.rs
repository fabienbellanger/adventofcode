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
    println!("Part 2 result: {}", part2(get_data(&input)));
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

fn part2(data: (HashMap<usize, Rule>, Vec<String>)) -> usize {
    let mut rules = data.0;
    rules.insert(8, Rule {
        val: String::from(""),
        rules: vec![vec![42], vec![42, 8]],
    });
    rules.insert(11, Rule {
        val: String::from(""),
        rules: vec![vec![42, 31], vec![42, 11, 31]],
    });

    let possible_messages = get_all_messages(&rules, 0, &vec![]);

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

    // let input = fs::read_to_string("input.txt").expect("Cannot find input.txt");
    // assert_eq!(182, part1(get_data(&input)));
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

#[test]
fn test_part2() {
    let input = r#"
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
    assert_eq!(12, part2(get_data(input)));

    // let input = fs::read_to_string("input.txt").expect("Cannot find input.txt");
    // assert_eq!(182, part1(get_data(&input)));
}
