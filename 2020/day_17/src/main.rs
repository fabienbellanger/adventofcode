use std::{collections::HashSet, fmt, fs};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self {x, y, z}
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cube(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
}

fn part1(active_cubes: HashSet<Cube>) -> usize {
    let mut min = Cube::new(isize::MAX, isize::MAX, isize::MAX);
    let mut max = Cube::new(isize::MIN, isize::MIN, isize::MIN);
    init_bounds(&active_cubes, &mut min, &mut max);

    let mut active_cubes = active_cubes.clone();
    
    for _cycle in 1..=6 {
        let mut cubes = active_cubes.clone();

        for x in min.x - 1..=max.x + 1 {
            for y in min.y - 1..=max.y + 1 {
                for z in min.z - 1..=max.z + 1 {
                    let cube = Cube::new(x, y, z);
                    let active_neighbors = count_active_neighbors(&active_cubes, &cube);

                    if cubes.contains(&cube) {
                        if !(active_neighbors == 2 || active_neighbors == 3) {
                            // Become inactive
                            cubes.remove(&cube);
                        }
                    } else {
                        if active_neighbors == 3 {
                            // Become active
                            cubes.insert(cube.clone());
                            update_bounds(&cube, &mut min, &mut max);
                        }
                    }
                }
            }
        }

        active_cubes = cubes;

    }

    active_cubes.len()
}

fn count_active_neighbors(active_cubes: &HashSet<Cube>, cube: &Cube) -> usize {
    let mut count = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                // Cube itself
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }

                if active_cubes.contains(&Cube::new(cube.x + x, cube.y + y, cube.z + z)) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn init_bounds(cubes: &HashSet<Cube>, min: &mut Cube, max: &mut Cube) {
    for cube in cubes {
        update_bounds(&cube, min, max);
    }
}

fn update_bounds(cube: &Cube, mut min: &mut Cube, mut max: &mut Cube) {
    if cube.x < min.x {
        min.x = cube.x;
    }
    if cube.y < min.y {
        min.y = cube.y;
    }
    if cube.z < min.z {
        min.z = cube.z;
    }
    if cube.x > max.x {
        max.x = cube.x;
    }
    if cube.y > max.y {
        max.y = cube.y;
    }
    if cube.z > max.z {
        max.z = cube.z;
    }
}

fn get_data(file: &str) -> HashSet<Cube> {
    fs::read_to_string(file).expect("Cannot read file")
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| {
                    Cube::new(x as isize, y as isize, 0)
                })
                .collect::<HashSet<Cube>>()
        })
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(112, part1(get_data("test.txt")));
    assert_eq!(313, part1(get_data("input.txt")));
}
