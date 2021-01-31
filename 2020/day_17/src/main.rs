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

#[derive(Clone, PartialEq, Eq, Hash)]
struct HyperCube {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl HyperCube {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self {x, y, z, w}
    }
}

impl fmt::Display for HyperCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x={}, y={}, z={}, w={})", self.x, self.y, self.z, self.w)
    }
}

impl fmt::Debug for HyperCube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HyperCube(x={}, y={}, z={}, w={})", self.x, self.y, self.z, self.w)
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_cube_data("input.txt")));
    println!("Part 2 result: {}", part2(get_hypercube_data("input.txt")));
}

fn part1(active_cubes: HashSet<Cube>) -> usize {
    let mut min = Cube::new(isize::MAX, isize::MAX, isize::MAX);
    let mut max = Cube::new(isize::MIN, isize::MIN, isize::MIN);
    init_cube_bounds(&active_cubes, &mut min, &mut max);

    let mut active_cubes = active_cubes.clone();
    
    for _cycle in 1..=6 {
        let mut cubes = active_cubes.clone();

        for x in min.x - 1..=max.x + 1 {
            for y in min.y - 1..=max.y + 1 {
                for z in min.z - 1..=max.z + 1 {
                    let cube = Cube::new(x, y, z);
                    let active_neighbors = count_active_cube_neighbors(&active_cubes, &cube);

                    if cubes.contains(&cube) {
                        if !(active_neighbors == 2 || active_neighbors == 3) {
                            // Become inactive
                            cubes.remove(&cube);
                        }
                    } else {
                        if active_neighbors == 3 {
                            // Become active
                            cubes.insert(cube.clone());
                            update_cube_bounds(&cube, &mut min, &mut max);
                        }
                    }
                }
            }
        }

        active_cubes = cubes;

    }

    active_cubes.len()
}

fn part2(active_hypercubes: HashSet<HyperCube>) -> usize {
    let mut min = HyperCube::new(isize::MAX, isize::MAX, isize::MAX, isize::MAX);
    let mut max = HyperCube::new(isize::MIN, isize::MIN, isize::MIN, isize::MIN);
    init_hypercube_bounds(&active_hypercubes, &mut min, &mut max);

    let mut active_hypercubes = active_hypercubes.clone();
    
    for _cycle in 1..=6 {
        let mut hypercubes = active_hypercubes.clone();

        for x in min.x - 1..=max.x + 1 {
            for y in min.y - 1..=max.y + 1 {
                for z in min.z - 1..=max.z + 1 {
                    for w in min.z - 1..=max.z + 1 {
                        let hypercube = HyperCube::new(x, y, z, w);
                        let active_neighbors = count_active_hypercube_neighbors(&active_hypercubes, &hypercube);

                        if hypercubes.contains(&hypercube) {
                            if !(active_neighbors == 2 || active_neighbors == 3) {
                                // Become inactive
                                hypercubes.remove(&hypercube);
                            }
                        } else {
                            if active_neighbors == 3 {
                                // Become active
                                hypercubes.insert(hypercube.clone());
                                update_hypercube_bounds(&hypercube, &mut min, &mut max);
                            }
                        }
                    }
                }
            }
        }

        active_hypercubes = hypercubes;

    }

    active_hypercubes.len()
}

fn count_active_cube_neighbors(active_cubes: &HashSet<Cube>, cube: &Cube) -> usize {
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

fn init_cube_bounds(cubes: &HashSet<Cube>, min: &mut Cube, max: &mut Cube) {
    for cube in cubes {
        update_cube_bounds(&cube, min, max);
    }
}

fn update_cube_bounds(cube: &Cube, mut min: &mut Cube, mut max: &mut Cube) {
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

fn count_active_hypercube_neighbors(active_hypercubes: &HashSet<HyperCube>, hypercube: &HyperCube) -> usize {
    let mut count = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    // Hypercube itself
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }

                    if active_hypercubes.contains(&HyperCube::new(
                        hypercube.x + x,
                        hypercube.y + y,
                        hypercube.z + z,
                        hypercube.w + w)) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn init_hypercube_bounds(hypercubes: &HashSet<HyperCube>, min: &mut HyperCube, max: &mut HyperCube) {
    for hypercube in hypercubes {
        update_hypercube_bounds(&hypercube, min, max);
    }
}

fn update_hypercube_bounds(hypercube: &HyperCube, mut min: &mut HyperCube, mut max: &mut HyperCube) {
    if hypercube.x < min.x {
        min.x = hypercube.x;
    }
    if hypercube.y < min.y {
        min.y = hypercube.y;
    }
    if hypercube.z < min.z {
        min.z = hypercube.z;
    }
    if hypercube.w < min.w {
        min.w = hypercube.w;
    }
    if hypercube.x > max.x {
        max.x = hypercube.x;
    }
    if hypercube.y > max.y {
        max.y = hypercube.y;
    }
    if hypercube.z > max.z {
        max.z = hypercube.z;
    }
    if hypercube.w > max.w {
        max.w = hypercube.w;
    }
}

fn get_cube_data(file: &str) -> HashSet<Cube> {
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

fn get_hypercube_data(file: &str) -> HashSet<HyperCube> {
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
                    HyperCube::new(x as isize, y as isize, 0, 0)
                })
                .collect::<HashSet<HyperCube>>()
        })
        .collect()
}

#[test]
fn test_part1() {
    assert_eq!(112, part1(get_cube_data("test.txt")));
    assert_eq!(313, part1(get_cube_data("input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(848, part2(get_hypercube_data("test.txt")));
    assert_eq!(2640, part1(get_cube_data("input.txt")));
}
