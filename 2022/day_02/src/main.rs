use std::fs;

fn main() {
    // println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<Vec<usize>>) {
    
}

fn part2(data: Vec<Vec<usize>>) {
    
}

// #[test]
// fn test_part1() {
//     assert_eq!(24_000, part1(get_data("test.txt")));
//     assert_eq!(1139, part1(get_data("input.txt")));
// }

// #[test]
// fn test_part2() {
//     assert_eq!(45_000, part2(get_data("test.txt")));
//     assert_eq!(1139, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Vec<Vec<usize>> {
    let parts = fs::read_to_string(file)
        .expect("Cannot read the file input.txt");
    
    vec![]
}
