use std::fs;

fn main() {
    println!("Part 1 result: {}", part1(get_data(), 25));
}

fn part1(data: Vec<usize>, preamble: usize) -> usize {
    let mut result = 0;

    for i in preamble..data.len() {
        let mut ok = false;
        'sum: for j in i-preamble..i {
            for k in j+1..i {
                let sum = data[j] + data[k];
                if sum == data[i] {
                    ok = true;
                    break 'sum;
                }
            }
        }

        if !ok {
            result = data[i];
            break;
        }
    }

    result
}

fn get_data() -> Vec<usize> {
    fs::read_to_string("input.txt")
        .expect("Cannot read the file input.txt")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn _get_data_test() -> Vec<usize> {
    fs::read_to_string("test.txt")
        .expect("Cannot read the file test.txt")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(127, part1(_get_data_test(), 5));
    assert_eq!(530627549, part1(get_data(), 25));
}
