use std::{collections::HashSet, fs};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pixel {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Picture(HashSet<Pixel>);

impl Picture {
    fn get_bounds(&self) -> (Pixel, Pixel) {
        let mut min = Pixel {
            x: isize::MAX,
            y: isize::MAX,
        };
        let mut max = Pixel {
            x: isize::MIN,
            y: isize::MIN,
        };

        for pixel in self.0.iter() {
            if pixel.x < min.x {
                min.x = pixel.x;
            }
            if pixel.x > max.x {
                max.x = pixel.x;
            }
            if pixel.y < min.y {
                min.y = pixel.y;
            }
            if pixel.y > max.y {
                max.y = pixel.y;
            }
        }

        (min, max)
    }

    fn _print(&self) {
        let (pixel_min, pixel_max) = self.get_bounds();

        for y in pixel_min.y..=pixel_max.y {
            for x in pixel_min.x..=pixel_max.x {
                match self.0.contains(&Pixel { x, y }) {
                    true => print!("#"),
                    _ => print!("."),
                }
            }
            println!();
        }
    }

    fn enhance(self, algo: &HashSet<usize>, has_border: bool) -> Self {
        let (pixel_min, pixel_max) = self.get_bounds();

        let mut new_picture = Picture(HashSet::new());
        let directions = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for y in pixel_min.y - 1..=pixel_max.y + 1 {
            for x in pixel_min.x - 1..=pixel_max.x + 1 {
                let mut number = String::new();

                for (dx, dy) in directions {
                    let new_x = x + dx;
                    let new_y = y + dy;

                    if new_x < pixel_min.x || new_x > pixel_max.x || new_y < pixel_min.y || new_y > pixel_max.y {
                        // Outside bounds
                        number.push(match has_border {
                            true => '1',
                            _ => '0',
                        });
                    } else {
                        // Inside bounds
                        number.push(match self.0.contains(&Pixel { x: new_x, y: new_y }) {
                            true => '1',
                            _ => '0',
                        });
                    }
                }

                // Convert binary to decimal
                let number = usize::from_str_radix(&number, 2).unwrap();

                if algo.contains(&number) {
                    new_picture.0.insert(Pixel { x, y });
                }
            }
        }

        new_picture
    }

    fn process(self, algo: HashSet<usize>, times: usize) -> usize {
        let inverse_color = algo.contains(&0) && !algo.contains(&511);
        let mut has_border = false;
        let mut picture = self;

        for _ in 0..times {
            picture = picture.enhance(&algo, has_border);
            if inverse_color {
                has_border = !has_border;
            }
        }

        picture.0.len()
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data("input.txt")));
    println!("Part 2 result: {}", part2(get_data("input.txt")));
}

fn part1(data: (HashSet<usize>, Picture)) -> usize {
    data.1.process(data.0, 2)
}

fn part2(data: (HashSet<usize>, Picture)) -> usize {
    data.1.process(data.0, 50)
}

#[test]
fn test_part1() {
    assert_eq!(35, part1(get_data("test.txt")));
    assert_eq!(5583, part1(get_data("input.txt"))); // Not 5565 nor 5586 :(
}

#[test]
fn test_part2() {
    assert_eq!(3351, part2(get_data("test.txt")));
    assert_eq!(19592, part2(get_data("input.txt")));
}

fn get_data(file: &str) -> (HashSet<usize>, Picture) {
    let data = fs::read_to_string(file).unwrap_or_else(|_| panic!("Cannot read the file {}", file));
    let (algo, picture) = data.split_once("\n\n").unwrap();

    let algo: HashSet<usize> = algo
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>()
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i)
        .collect();

    let picture: HashSet<Pixel> = picture
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| Pixel {
                    x: x as isize,
                    y: y as isize,
                })
                .collect::<HashSet<Pixel>>()
        })
        .flatten()
        .collect();

    (algo, Picture(picture))
}
