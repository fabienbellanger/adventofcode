use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Cannot find input.txt");

    // println!("Part 1 result: {}", part1(get_data(&input)));
    // println!("Part 2 result: {}", part2(get_data(&input)));
}

fn part1() -> usize {
    0
}

fn get_data(data: &str) {
}

#[test]
fn test_part1() {
    let _input = r#"
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
    assert_eq!(2, part1());
}
