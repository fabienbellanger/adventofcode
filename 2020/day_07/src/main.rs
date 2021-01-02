use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
}

fn part1(data: HashMap<String, HashMap<String, usize>>) -> usize {
    let mut result = 0;

    for (_bag, bags) in &data {
        let found = find_shiny_gold(&data, bags);
        if found{
            result += 1;
        }
    }

    result
}

fn find_shiny_gold(data: &HashMap<String, HashMap<String, usize>>, bags: &HashMap<String, usize>) -> bool {
    let mut found = false;

    if bags.contains_key("shiny gold") {
        return true;
    }

    for (b, _v) in bags {
        found = find_shiny_gold(data, &data[b]);
        if found {
            return found;
        }
    }

    found
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
                let mut parts_next = b.split(" ");
                let val = parts_next.next().unwrap().parse().unwrap();
                let mut bag_name = parts_next.next().unwrap().to_owned();
                bag_name.push_str(" ");
                bag_name.push_str(parts_next.next().unwrap());

                next_bags.insert(bag_name, val);
            }
        }

        bags.insert(main_bag.to_owned(), next_bags);
    }

    bags
}

fn get_data() -> HashMap<String, HashMap<String, usize>> {
    let data = fs::read_to_string("input.txt").expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.to_string())
        .collect();

    get_bags(data)
}

fn _get_data_test() -> HashMap<String, HashMap<String, usize>> {
    let data = fs::read_to_string("test.txt").expect("Cannot read the file test.txt")
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
