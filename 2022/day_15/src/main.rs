use std::fs;

// Distance de Manhattan
// Entre deux points A et B, de coordonnées respectives ( X A , Y A ) et ( X B , Y B ),
// la distance de Manhattan est définie par :
//
// d(A , B) = | X B − X A | + | Y B − Y A |

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    // println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: Vec<String>) -> usize {
    dbg!(&data);
    0
}

// fn part2(data: Vec<String>) -> usize {
//     0
// }

#[test]
fn test_part1() {
    assert_eq!(26, part1(get_data("test.txt")));
    // assert_eq!(0, part1(get_data("input.txt")));
}

// #[test]
// fn test_part2() {
//     assert_eq!(0, part2(get_data("test.txt")));
//     // assert_eq!(0, part2(get_data("input.txt")));
// }

fn get_data(file: &str) -> Vec<String> {
    fs::read_to_string(file)
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect()
}
