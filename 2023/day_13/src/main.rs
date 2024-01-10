use std::cmp::min;
use std::collections::HashSet;
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
        let mut y_reflections = Vec::new();
        let mut parts = Vec::new();

        for y in 0..self.height {
            let mut tmp = Vec::new();
            for point in self.points.iter() {
                if point.y == y as isize {
                    tmp.push(point.x);
                }
            }

            if current == tmp {
                y_reflections.push(y - 1);
            }
            parts.push(tmp.clone());
            current = tmp;
        }

        for y_reflection in y_reflections {
            let delta = min(y_reflection + 1, self.height - y_reflection - 1);

            let part1 = parts[y_reflection + 1 - delta..y_reflection + 1].to_vec();
            let mut part2 = parts[y_reflection + 1..y_reflection + delta + 1].to_vec();
            part2.reverse();

            if part1 == part2 {
                return Some(y_reflection);
            }
        }

        None
    }

    fn find_vertical_reflection(&self) -> Option<usize> {
        let mut current = Vec::new();
        let mut x_reflections = Vec::new();
        let mut parts = Vec::new();

        for x in 0..self.width {
            let mut tmp = Vec::new();
            for point in self.points.iter() {
                if point.x == x as isize {
                    tmp.push(point.y);
                }
            }

            if current == tmp {
                x_reflections.push(x - 1);
            }
            parts.push(tmp.clone());
            current = tmp;
        }

        for x_reflection in x_reflections {
            let delta = min(x_reflection + 1, self.width - x_reflection - 1);

            let part1 = parts[x_reflection + 1 - delta..x_reflection + 1].to_vec();
            let mut part2 = parts[x_reflection + 1..x_reflection + delta + 1].to_vec();
            part2.reverse();

            if part1 == part2 {
                return Some(x_reflection);
            }
        }

        None
    }

    fn difference_lines_count(l1: &[isize], l2: &[isize]) -> usize {
        let s1: HashSet<_> = l1.into_iter().collect();
        let s2: HashSet<_> = l2.into_iter().collect();

        s1.symmetric_difference(&s2).count()
    }

    fn find_reflexion_line(&self) -> Option<usize> {
        let mut current = Vec::new();
        let mut y_reflections = Vec::new();
        let mut parts = Vec::new();
        let mut chances = 1;

        for y in 0..self.height {
            let mut tmp = Vec::new();

            for point in self.points.iter() {
                if point.y == y as isize {
                    tmp.push(point.x);
                }
            }

            // Number of differences between two lines
            let difference = Self::difference_lines_count(&current, &tmp);
            if difference <= 1 {
                y_reflections.push((y - 1, difference));
            }

            parts.push(tmp.clone());
            current = tmp;
        }

        dbg!(&y_reflections);

        None
    }
}

fn part1(data: Vec<Grid>) -> usize {
    let mut result = 0;

    for (i, grid) in data.iter().enumerate() {
        result += match (grid.find_horizontal_reflection(), grid.find_vertical_reflection()) {
            (Some(row), None) => (row + 1) * 100,
            (None, Some(col)) => col + 1,
            (r, c) => panic!("invalid reflection {r:?} {c:?} at {i}"),
        };
    }

    result
}

fn part2(data: Vec<Grid>) -> usize {
    let mut result = 0;

    for (i, grid) in data.iter().enumerate() {
        result += if let Some(row) = grid.find_reflexion_line() {
            (row + 1) * 100
        } else {
            0 // panic!("invalid reflection line at {i}")
        };
    }

    result
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
                        .filter(|(_, c)| *c == '#')
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
        assert_eq!(29_130, part1(parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(400, part2(parse_input(TEST)));
        // assert_eq!(0, part2(parse_input(INPUT)));
    }
}
