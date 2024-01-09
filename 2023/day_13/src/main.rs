use std::cmp::min;
use std::fs;
use utils::point::Point;

const INPUT: &str = "input.txt";

fn main() {
    println!("Part 1 result: {}", part1(parse_input(INPUT)));
    println!("Part 2 result: {}", part2(parse_input(INPUT)));
}

#[derive(Debug, Clone)]
struct Grid {
    points: Vec<Point>,
    width: usize,
    height: usize,
}

impl Grid {
    fn find_horizontal_reflection(&self) -> Option<usize> {
        let mut current = Vec::new();
        let mut y_reflection = None;

        for y in 0..self.height {
            let mut tmp = Vec::new();
            for point in self.points.iter() {
                if point.y == y as isize {
                    tmp.push(point.x);
                }
            }

            if current == tmp {
                y_reflection = Some(y - 1);
            }
            current = tmp;
        }

        if let Some(y_reflection) = y_reflection {
            dbg!(self.height, y_reflection);
            // let y_len = min(self.height - y_reflection, part.len());

            // let mut part2 = (&parts[y_reflection..y_reflection + y_len]).to_vec();
            // part2.reverse();
            // if part == part2 {
            //     return Some(y_reflection);
            // }
        }

        None
    }

    fn find_vertical_reflection(&self) -> Option<usize> {
        None
    }
}

fn part1(data: Vec<Grid>) -> usize {
    // dbg!(&data);
    for grid in data.iter() {
        dbg!(grid.find_horizontal_reflection());
    }
    todo!()
}

fn part2(data: Vec<Grid>) -> usize {
    todo!()
}

fn parse_input(file: &str) -> Vec<Grid> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {file}"))
        .trim()
        .split("\n\n")
        .into_iter()
        .map(|grid| {
            let mut width = 0;
            let mut height = 0;

            let points = grid
                .trim()
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    if y == 0 {
                        width = line.trim().len();
                    }
                    height += 1;

                    line.chars()
                        .enumerate()
                        .filter(|(x, c)| *c == '#')
                        .map(|(x, _)| Point::new(x as isize, y as isize))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();
            Grid { points, width, height }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(405, part1(parse_input(TEST)));
        // assert_eq!(0, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        // assert_eq!(0, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
