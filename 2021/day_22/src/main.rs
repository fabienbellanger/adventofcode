use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone)]
struct Step {
    turn_on: bool,
    cuboid: Cuboid,
}

#[derive(Debug, Clone)]
struct Cuboid {
    min: Cube,
    max: Cube,
}

impl Cuboid {
    fn cubes_number(&self) -> usize {
        ((self.max.x - self.min.x).abs() as usize + 1).pow(3)
    }

    fn contains(&self, cude: &Cube) -> bool {
        todo!()
    }

    fn intersection(&self, cuboid: &Cuboid) {
        // 1. Select min cuboid
        // 2. Check for each cube if in larger cuboid
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(steps: Vec<Step>) -> usize {
    let mut cubes_on: HashSet<Cube> = HashSet::new();

    // Limits
    let min_cube = Cube { x: -50, y: -50, z: -50 };
    let max_cube = Cube { x: 50, y: 50, z: 50 };

    for step in steps {
        let cuboid = step.cuboid;

        if cuboid.min.x >= min_cube.x
            && cuboid.min.y >= min_cube.y
            && cuboid.min.z >= min_cube.z
            && cuboid.max.x <= max_cube.x
            && cuboid.max.y <= max_cube.y
            && cuboid.max.z <= max_cube.z
        {
            for x in cuboid.min.x..=cuboid.max.x {
                for y in cuboid.min.y..=cuboid.max.y {
                    for z in cuboid.min.z..=cuboid.max.z {
                        if step.turn_on {
                            cubes_on.insert(Cube { x, y, z });
                        } else {
                            cubes_on.remove(&Cube { x, y, z });
                        }
                    }
                }
            }
        }
    }

    cubes_on.len()
}

fn part2(steps: Vec<Step>) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(39, part1(get_data("test.txt")));
    assert_eq!(474140, part1(get_data("test_big.txt")));
    assert_eq!(570915, part1(get_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(2758514936282235, part2(get_data("test_big.txt")));
    // assert_eq!(0, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> Vec<Step> {
    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("Cannot read the file {}", file))
        .trim()
        .lines()
        .map(|line| -> Step {
            let (action, coords) = line.trim().split_once(' ').unwrap();

            let mut axis = coords.trim().split(',');
            let (x_min, x_max) = axis.next().unwrap().trim_start_matches("x=").split_once("..").unwrap();
            let (y_min, y_max) = axis.next().unwrap().trim_start_matches("y=").split_once("..").unwrap();
            let (z_min, z_max) = axis.next().unwrap().trim_start_matches("z=").split_once("..").unwrap();

            Step {
                turn_on: action == "on",
                cuboid: Cuboid {
                    min: Cube {
                        x: x_min.parse().unwrap(),
                        y: y_min.parse().unwrap(),
                        z: z_min.parse().unwrap(),
                    },
                    max: Cube {
                        x: x_max.parse().unwrap(),
                        y: y_max.parse().unwrap(),
                        z: z_max.parse().unwrap(),
                    },
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cubes_number_in_cuboid() {
        let cuboid = Cuboid {
            min: Cube { x: 10, y: 10, z: 10 },
            max: Cube { x: 12, y: 12, z: 12 },
        };
        assert_eq!(27, cuboid.cubes_number());

        let cuboid = Cuboid {
            min: Cube { x: -10, y: -10, z: -10 },
            max: Cube { x: -10, y: -10, z: -10 },
        };
        assert_eq!(1, cuboid.cubes_number());
    }
}
