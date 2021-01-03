use std::collections::HashMap;
use std::fs;

const SHINY_GOLD_BAG: &str = "shiny gold";

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(data: HashMap<String, HashMap<String, usize>>) -> usize {
    let mut result = 0;

    for bag in data.keys() {
        if find_shiny_gold(&data, bag) {
            result += 1;
        }
    }

    result
}

fn part2(data: HashMap<String, HashMap<String, usize>>) -> usize {
    let mut result = 0;

    let shiny_gold = data.get(SHINY_GOLD_BAG).unwrap();
    for (bag, nb) in shiny_gold {
        result += nb * count_shiny_gold(&data, &bag);
    }

    result
}

fn find_shiny_gold(data: &HashMap<String, HashMap<String, usize>>, bag: &str) -> bool {
    for b in data.get(bag).unwrap().keys() {
        if b == SHINY_GOLD_BAG || find_shiny_gold(data, b) {
            return true;
        }
    }

    false
}

fn count_shiny_gold(data: &HashMap<String, HashMap<String, usize>>, bag: &str) -> usize {
    let mut s = 1;

    for (b, v) in data.get(bag).unwrap() {
        s += v * count_shiny_gold(data, b);
    }

    s
}

fn get_bags(data: Vec<String>) -> HashMap<String, HashMap<String, usize>> {
    // + light red bags contain 1 bright white bag, 2 muted yellow bags.
    // + dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    // + bright white bags contain 1 shiny gold bag.
    // + muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    // - shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    // - dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    // - vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    // - faded blue bags contain no other bags.
    // - dotted black bags contain no other bags.

    let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in data {
        let mut parts = line.split(" bags contain ");
        let main_bag = parts.next().unwrap();
        let other_bags = parts.next().unwrap();
        let mut next_bags: HashMap<String, usize> = HashMap::new();

        if other_bags != "no other bags." {
            let parts_other: Vec<&str> = other_bags.split(", ").collect();

            for b in parts_other {
                let mut parts_next = b.split_whitespace();
                let val = parts_next.next().unwrap().parse().unwrap();
                let mut bag_name = parts_next.next().unwrap().to_owned();
                bag_name.push(' ');
                bag_name.push_str(parts_next.next().unwrap());

                next_bags.insert(bag_name, val);
            }
        }

        bags.insert(main_bag.to_owned(), next_bags);
    }

    bags
}

fn get_data() -> HashMap<String, HashMap<String, usize>> {
    let data = fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect();

    get_bags(data)
}

fn _get_data_test() -> HashMap<String, HashMap<String, usize>> {
    let data = fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect();

    get_bags(data)
}

fn _get_data_test_2() -> HashMap<String, HashMap<String, usize>> {
    let data = fs::read_to_string("test_2.txt")
        .expect("Cannot read the file test_2.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect();

    get_bags(data)
}

#[test]
fn test_part1() {
    assert_eq!(4, part1(_get_data_test()));
    assert_eq!(179, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(32, part2(_get_data_test()));
    assert_eq!(126, part2(_get_data_test_2()));
    assert_eq!(18925, part2(get_data()));
}
