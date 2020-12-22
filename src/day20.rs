use std::{collections::HashMap, fmt::Display};

struct Tile {
    id: u16,
    pixels: Vec<Vec<char>>,
    borders: [String; 4],
}

impl Tile {
    fn new(input: &str) -> Self {
        let mut parts = input.splitn(2, '\n');
        let id = parts
            .next()
            .unwrap()
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse::<u16>()
            .unwrap();
        let pixels = parts
            .next()
            .unwrap()
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let borders = [
            // top
            pixels[0].iter().collect(),
            // right
            pixels.iter().map(|line| line[line.len() - 1]).collect(),
            // bottom
            pixels[pixels.len() - 1].iter().collect(),
            // left
            pixels.iter().map(|line| line[0]).collect(),
        ];
        Self {
            id,
            pixels,
            borders,
        }
    }

    fn try_assemble(&self, other: &mut Self) -> Option<usize> {
        let mut found = None;
        'look: for (index, border) in self.borders.iter().enumerate() {
            for (other_index, other_border) in other.borders.iter().enumerate() {
                if border == other_border
                    || border == &other_border.chars().rev().collect::<String>()
                {
                    found = Some((index, other_index));
                    break 'look;
                }
            }
        }
        if let Some((index, other_index)) = found {
            // println!("before\n{}", other);
            match (4 + other_index - index) % 4 {
                0 => {
                    if index % 2 == 0 {
                        other.flip_vert();
                    } else {
                        other.flip_hor();
                    }
                }
                1 => {
                    other.rotate_right();
                }
                2 => {}
                3 => {
                    other.rotate_right();
                    if index % 2 == 0 {
                        other.flip_vert();
                    } else {
                        other.flip_hor();
                    }
                }
                _ => {}
            }
            if other.borders[(index + 2) % 4] != self.borders[index] {
                if index % 2 == 0 {
                    other.flip_hor();
                } else {
                    other.flip_vert();
                }
            }
            // println!("after\n{}", other);
            return Some(index);
        }
        None
    }

    fn rotate_right(&mut self) {
        self.borders.rotate_right(1);
        self.borders[0] = self.borders[0].chars().rev().collect();
        self.borders[2] = self.borders[2].chars().rev().collect();
        let mut lines = vec![];
        for i in 0..self.pixels.len() {
            let mut line = vec![];
            for j in 0..self.pixels[i].len() {
                line.push(self.pixels[self.pixels.len() - 1 - j][i]);
            }
            lines.push(line);
        }
        self.pixels = lines;
    }

    fn flip_vert(&mut self) {
        self.borders.swap(0, 2);
        self.borders[1] = self.borders[1].chars().rev().collect();
        self.borders[3] = self.borders[3].chars().rev().collect();
        let length = self.pixels.len();
        for i in 0..length / 2 {
            self.pixels.swap(i, length - 1 - i);
        }
    }

    fn flip_hor(&mut self) {
        self.borders.swap(1, 3);
        self.borders[0] = self.borders[0].chars().rev().collect();
        self.borders[2] = self.borders[2].chars().rev().collect();
        for line in self.pixels.iter_mut() {
            let length = line.len();
            for i in 0..length / 2 {
                line.swap(i, length - 1 - i);
            }
        }
    }

    fn count_sea_monsters(&self) -> u32 {
        let mut count = 0;
        for i in 1..self.pixels.len() - 1 {
            for j in 0..=self.pixels[i].len() - 20 {
                if self.pixels[i][j] == '#'
                    && self.pixels[i][j + 5] == '#'
                    && self.pixels[i][j + 6] == '#'
                    && self.pixels[i][j + 11] == '#'
                    && self.pixels[i][j + 12] == '#'
                    && self.pixels[i][j + 17] == '#'
                    && self.pixels[i][j + 18] == '#'
                    && self.pixels[i][j + 19] == '#'
                    && self.pixels[i - 1][j + 18] == '#'
                    && self.pixels[i + 1][j + 1] == '#'
                    && self.pixels[i + 1][j + 4] == '#'
                    && self.pixels[i + 1][j + 7] == '#'
                    && self.pixels[i + 1][j + 10] == '#'
                    && self.pixels[i + 1][j + 13] == '#'
                    && self.pixels[i + 1][j + 16] == '#'
                {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "id: {}", self.id)?;
        for line in &self.pixels {
            for pixel in line {
                write!(f, "{}", pixel)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct Picture {
    tiles: HashMap<(i8, i8), Tile>,
    boundaries: ((i8, i8), (i8, i8)),
}

impl Picture {
    fn new(mut tiles: Vec<Tile>) -> Self {
        let mut picture = HashMap::<(i8, i8), Tile>::new();
        let mut boundaries = ((0i8, 0i8), (0i8, 0i8));

        let first_tile = tiles.pop().unwrap();
        picture.insert((0, 0), first_tile);

        let mut to_process = vec![(0i8, 0i8)];
        while to_process.len() > 0 {
            let mut new_to_process = vec![];
            for coordinates in &to_process {
                let tile = picture.get(&coordinates).unwrap();
                // println!(
                //     "tile to process\n{:?}\n{}\n{:?}",
                //     coordinates, tile, tile.borders
                // );
                let mut matching_tiles = vec![];
                for (index, other_tile) in tiles.iter_mut().enumerate() {
                    if let Some(direction) = tile.try_assemble(other_tile) {
                        matching_tiles.push((index, direction));
                        if matching_tiles.len() == 4 {
                            break;
                        }
                    }
                }
                for &(index, direction) in matching_tiles.iter().rev() {
                    let matching_tile = tiles.remove(index);
                    let (x, y) = match direction {
                        0 => (coordinates.0 - 1, coordinates.1),
                        1 => (coordinates.0, coordinates.1 + 1),
                        2 => (coordinates.0 + 1, coordinates.1),
                        3 => (coordinates.0, coordinates.1 - 1),
                        _ => panic!("invalid direction"),
                    };
                    // println!(
                    //     "matching tile\n{:?}\n{}\n{:?}",
                    //     (x, y),
                    //     matching_tile,
                    //     matching_tile.borders
                    // );
                    picture.insert((x, y), matching_tile);
                    new_to_process.push((x, y));
                    boundaries = (
                        (
                            if x < boundaries.0 .0 {
                                x
                            } else {
                                boundaries.0 .0
                            },
                            if y < boundaries.0 .1 {
                                y
                            } else {
                                boundaries.0 .1
                            },
                        ),
                        (
                            if x > boundaries.1 .0 {
                                x
                            } else {
                                boundaries.1 .0
                            },
                            if y > boundaries.1 .1 {
                                y
                            } else {
                                boundaries.1 .1
                            },
                        ),
                    )
                }
            }
            to_process = new_to_process;
        }

        Self {
            tiles: picture,
            boundaries,
        }
    }

    fn get_picture(&self) -> Tile {
        let mut tiles = vec![];
        for x in (self.boundaries.0 .0)..=(self.boundaries.1 .0) {
            let mut row = vec![];
            for y in (self.boundaries.0 .1)..=(self.boundaries.1 .1) {
                let tile = self.tiles.get(&(x, y)).unwrap();
                row.push(tile);
            }
            tiles.push(row);
        }

        let tile_height = tiles[0][0].pixels.len();
        let tile_width = tiles[0][0].pixels[0].len();
        let mut pixels = vec![];
        for row_index in 0..tiles.len() * (tile_height - 2) {
            let mut row = vec![];
            for column_index in 0..tiles[row_index / (tile_height - 2)].len() * (tile_width - 2) {
                row.push(
                    tiles[row_index / (tile_height - 2)][column_index / (tile_width - 2)].pixels
                        [row_index % (tile_height - 2) + 1][column_index % (tile_width - 2) + 1],
                )
            }
            pixels.push(row);
        }

        Tile {
            pixels,
            borders: [String::new(), String::new(), String::new(), String::new()],
            id: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<Tile> {
    input.split("\n\n").map(Tile::new).collect()
}

fn solve_1(picture: &Picture) -> u64 {
    let ((xmin, ymin), (xmax, ymax)) = picture.boundaries;
    picture.tiles.get(&(xmin, ymin)).unwrap().id as u64
        * picture.tiles.get(&(xmin, ymax)).unwrap().id as u64
        * picture.tiles.get(&(xmax, ymax)).unwrap().id as u64
        * picture.tiles.get(&(xmax, ymin)).unwrap().id as u64
}

fn solve_2(mut tile: Tile) -> u32 {
    let total = tile.pixels.iter().fold(0u32, |acc, curr| {
        acc + curr
            .iter()
            .fold(0, |acc2, curr2| if *curr2 == '#' { acc2 + 1 } else { acc2 })
    });
    let mut sea_monsters = 0;
    'transform: for f in 0..2 {
        for r in 0..4 {
            sea_monsters = tile.count_sea_monsters();
            if sea_monsters != 0 {
                break 'transform;
            }
            tile.rotate_right();
        }
        tile.flip_hor();
    }
    total - (sea_monsters * 15)
}

pub fn run() {
    // try to find four tiles that have 2 side that don't match other tiles
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day20").unwrap();
    let tiles = parse_input(&input);
    let picture = Picture::new(tiles);
    println!(
        "day 20 solution 1 : {}, {}us",
        solve_1(&picture),
        timer.elapsed().as_micros()
    );
    let mut picture = picture.get_picture();
    picture.flip_hor();
    picture.flip_vert();
    println!(
        "day 20 solution 2 : {}, {}us",
        solve_2(picture),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Tile 0000:
.####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.##..
#.#.##.###.#.##.##.#####
..##.###.####..#.####.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.######.
.###.###.#######..#####.
..##.#..#..#.#######.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#";
        let tile = Tile::new(input);
        assert_eq!(solve_2(tile), 273);
    }
}
