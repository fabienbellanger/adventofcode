use std::fs;

const INPUT: &str = "input.txt";

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

fn hash_string(s: &str) -> usize {
    s.chars().fold(0, |acc, c| {
        let mut h = acc + (c as u8) as usize;
        h *= 17;
        h %= 256;

        h
    })
}

fn parse_step(s: &str) -> (&str, Option<usize>) {
    if s.contains('-') {
        let label = s.strip_suffix('-');
        if let Some(label) = label {
            return (label, None);
        }
    } else {
        let parts = s.split_once('=');
        if let Some((label, focal_length)) = parts {
            let focal_length = focal_length.parse::<usize>();
            if let Ok(focal_length) = focal_length {
                return (label, Some(focal_length));
            }
        }
    }

    ("", None)
}

fn part1(data: Vec<String>) -> usize {
    data.iter().map(|s| hash_string(s)).sum()
}

fn part2(data: Vec<String>) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = (0..256).map(|_| Vec::new()).collect();

    for step in data.iter() {
        let (label, focal_length) = parse_step(step);

        if label.is_empty() {
            panic!("invalid step: {step}");
        }

        let box_number = hash_string(label);
        let position = boxes[box_number].iter().position(|(l, _)| *l == label);

        if let Some(focal_length) = focal_length {
            // Add or modify
            if let Some(position) = position {
                boxes[box_number][position].1 = focal_length;
            } else {
                boxes[box_number].push((label, focal_length));
            }
        } else {
            // Remove
            if let Some(position) = position {
                boxes[box_number].remove(position);
            }
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .map(|(j, (_, f))| (i + 1) * (j + 1) * f)
                .sum::<usize>()
        })
        .sum()
}

fn parse_input(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {file}"))
        .trim()
        .split(',')
        .map(|part| part.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(1_320, part1(parse_input(TEST)));
        assert_eq!(519_041, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_parse_step() {
        assert_eq!(parse_step("rn=1"), ("rn", Some(1)));
        assert_eq!(parse_step("cm-"), ("cm", None));
        assert_eq!(parse_step("gfgd"), ("", None));
    }

    #[test]
    fn test_part2() {
        assert_eq!(145, part2(parse_input(TEST)));
        assert_eq!(260_530, part2(parse_input(INPUT)));
    }
}
