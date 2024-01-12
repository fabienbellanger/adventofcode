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
    fn difference_lines_count(l1: &[isize], l2: &[isize]) -> usize {
        let s1: HashSet<_> = l1.iter().collect();
        let s2: HashSet<_> = l2.iter().collect();

        s1.symmetric_difference(&s2).count()
    }

    fn find_reflection(&self, is_vertical: bool, is_part_1: bool) -> Option<usize> {
        let mut current = Vec::new();
        let mut reflections = Vec::new();
        let mut parts = Vec::new();
        let bound = match is_vertical {
            false => self.height,
            true => self.width,
        };
        let difference_max = match is_part_1 {
            false => 1,
            true => 0,
        };

        for i in 0..bound {
            let mut tmp = Vec::new();
            for point in self.points.iter() {
                if !is_vertical && point.y == i as isize {
                    tmp.push(point.x);
                } else if is_vertical && point.x == i as isize {
                    tmp.push(point.y);
                }
            }

            let difference = Self::difference_lines_count(&current, &tmp);
            if difference <= difference_max && !current.is_empty() {
                reflections.push(i - 1);
            }

            parts.push(tmp.clone());
            current = tmp;
        }

        for reflection in reflections {
            let delta = min(reflection + 1, bound - reflection - 1);

            let mut part1 = parts[reflection + 1 - delta..reflection + 1].to_vec();
            let part2 = parts[reflection + 1..reflection + delta + 1].to_vec();
            part1.reverse();

            if is_part_1 && part1 == part2 {
                return Some(reflection);
            } else if !is_part_1 {
                let mut smudge = 0;
                for j in 0..delta {
                    let diff = Self::difference_lines_count(&part1[j], &part2[j]);

                    smudge += diff;

                    if smudge > 1 {
                        break;
                    }
                }

                if smudge == 1 {
                    return Some(reflection);
                }
            }
        }

        None
    }
}

fn process(data: Vec<Grid>, is_part_1: bool) -> usize {
    let mut result = 0;

    for (i, grid) in data.iter().enumerate() {
        let r = match is_part_1 {
            true => (grid.find_reflection(false, true), grid.find_reflection(true, true)),
            false => (grid.find_reflection(false, false), grid.find_reflection(true, false)),
        };
        result += match r {
            (Some(row), None) => (row + 1) * 100,
            (None, Some(col)) => col + 1,
            (r, c) => panic!("invalid reflection {r:?} {c:?} at {i}"),
        };
    }

    result
}

fn part1(data: Vec<Grid>) -> usize {
    process(data, true)
}

fn part2(data: Vec<Grid>) -> usize {
    process(data, false)
}

fn parse_input(file: &str) -> Vec<Grid> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {file}"))
        .trim()
        .split("\n\n")
        .map(|grid| {
            let mut width = 0;
            let mut height = 0;

            let points = grid
                .trim()
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
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
        assert_eq!(33_438, part2(parse_input(INPUT)));
    }
}
