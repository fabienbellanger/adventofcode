#![allow(unused_variables)]

const INPUT: &str = "input.txt";

fn main() {
    println!("Part 1 result: {}", part1(parse_input_1(INPUT)));
    println!("Part 2 result: {}", part2(parse_input_2(INPUT)));
}

// Exemple : 7 => 9
// Push = 1, Reste = 7 - 1 = 6, Result = 6 * 1 = 6  => KO
// Push = 2, Reste = 7 - 2 = 5, Result = 5 * 2 = 10 => OK
// Push = 3, Reste = 7 - 3 = 4, Result = 4 * 3 = 12 => OK
// Push = 4, Reste = 7 - 4 = 3, Result = 3 * 4 = 12 => OK
// Push = 5, Reste = 7 - 6 = 2, Result = 2 * 5 = 10 => OK
// Push = 6, Reste = 7 - 7 = 1, Result = 1 * 6 = 6  => KO
// ==> Result = (N - Push) * Push
fn part1(data: (Vec<usize>, Vec<usize>)) -> usize {
    let time = data.0;
    let distance = data.1;
    let size = time.len();

    (0..size)
        .map(|race| {
            let n = time[race];
            (1..=n).map(|i| (n - i) * i).filter(|v| *v > distance[race]).count()
        })
        .product()
}

fn part2(data: (usize, usize)) -> usize {
    let n = data.0;

    (1..=n).map(|i| (n - i) * i).filter(|v| *v > data.1).count()
}

fn parse_input_1(file: &str) -> (Vec<usize>, Vec<usize>) {
    if file == INPUT {
        (vec![60, 80, 86, 76], vec![601, 1163, 1559, 1300])
    } else {
        (vec![7, 15, 30], vec![9, 40, 200])
    }
}

fn parse_input_2(file: &str) -> (usize, usize) {
    if file == INPUT {
        (60_808_676, 601_116_315_591_300)
    } else {
        (71_530, 940_200)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(288, part1(parse_input_1(TEST)));
        assert_eq!(1_155_175, part1(parse_input_1(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(71_503, part2(parse_input_2(TEST)));
        assert_eq!(35_961_505, part2(parse_input_2(INPUT)));
    }
}
