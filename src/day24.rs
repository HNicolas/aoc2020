use std::collections::{HashMap, HashSet};

fn get_adjacent_tiles(tile: &(i8, i8)) -> [(i8, i8); 6] {
    [
        // e, se, sw, w, nw, and ne
        (tile.0, tile.1 + 1),
        (
            tile.0 + 1,
            if tile.0 % 2 == 0 { tile.1 } else { tile.1 + 1 },
        ),
        (
            tile.0 + 1,
            if tile.0 % 2 == 0 { tile.1 - 1 } else { tile.1 },
        ),
        (tile.0, tile.1 - 1),
        (
            tile.0 - 1,
            if tile.0 % 2 == 0 { tile.1 - 1 } else { tile.1 },
        ),
        (
            tile.0 - 1,
            if tile.0 % 2 == 0 { tile.1 } else { tile.1 + 1 },
        ),
    ]
}

/* 001122
0  x x x
1   x x x
2  x x x
3   x x x
4  x x x
*/
fn get_destination_from_reference(reference: (i8, i8), directions: &str) -> (i8, i8) {
    let mut rest = directions;
    let mut destination = reference;
    while rest.len() > 0 {
        // e, se, sw, w, nw, and ne
        match rest {
            _ if rest.starts_with("e") => {
                rest = &rest[1..];
                destination.1 += 1;
            }
            _ if rest.starts_with("se") => {
                rest = &rest[2..];
                destination = (
                    destination.0 + 1,
                    if destination.0 % 2 == 0 {
                        destination.1
                    } else {
                        destination.1 + 1
                    },
                );
            }
            _ if rest.starts_with("sw") => {
                rest = &rest[2..];
                destination = (
                    destination.0 + 1,
                    if destination.0 % 2 == 0 {
                        destination.1 - 1
                    } else {
                        destination.1
                    },
                );
            }
            _ if rest.starts_with("w") => {
                rest = &rest[1..];
                destination.1 -= 1;
            }
            _ if rest.starts_with("nw") => {
                rest = &rest[2..];
                destination = (
                    destination.0 - 1,
                    if destination.0 % 2 == 0 {
                        destination.1 - 1
                    } else {
                        destination.1
                    },
                );
            }
            _ if rest.starts_with("ne") => {
                rest = &rest[2..];
                destination = (
                    destination.0 - 1,
                    if destination.0 % 2 == 0 {
                        destination.1
                    } else {
                        destination.1 + 1
                    },
                );
            }
            _ => panic!("invalid direction"),
        }
    }
    destination
}

struct Game {
    black_tiles: HashSet<(i8, i8)>,
}

impl Game {
    fn new(input: &str) -> Self {
        let black_tiles = input
            .lines()
            .fold(HashSet::<(i8, i8)>::new(), |mut set, directions| {
                let destination = get_destination_from_reference((0, 0), directions);
                if set.contains(&destination) {
                    set.remove(&destination);
                } else {
                    set.insert(destination);
                }
                set
            });
        Self { black_tiles }
    }

    fn count_black_tiles(&self) -> usize {
        self.black_tiles.len()
    }

    /// - Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    /// - Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    fn run_n_days(&mut self, days: u32) {
        for _ in 0..days {
            let mut adjacent_count = HashMap::new();
            for black_tile in self.black_tiles.iter() {
                for adjacent_tile in get_adjacent_tiles(black_tile).iter() {
                    if let Some(count) = adjacent_count.get_mut(adjacent_tile) {
                        *count += 1;
                    } else {
                        adjacent_count.insert(*adjacent_tile, 1u8);
                    }
                }
            }

            let mut new_black_tiles = HashSet::new();
            for (tile, count) in adjacent_count {
                if count == 2 || (self.black_tiles.contains(&tile) && count == 1) {
                    new_black_tiles.insert(tile);
                }
            }
            self.black_tiles = new_black_tiles;
        }
    }
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day24").unwrap();
    let mut game = Game::new(&input);
    println!(
        "day 24 solution 1 : {}, {}us",
        game.count_black_tiles(),
        timer.elapsed().as_micros()
    );
    game.run_n_days(100);
    println!(
        "day 24 solution 2 : {}, {}us",
        game.count_black_tiles(),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew";
        let game = Game::new(&input);
        assert_eq!(game.count_black_tiles(), 10);
    }

    #[test]
    fn test_solution_2() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw\nneeenesenwnwwswnenewnwwsewnenwseswesw\nseswneswswsenwwnwse\nnwnwneseeswswnenewneswwnewseswneseene\nswweswneswnenwsewnwneneseenw\neesenwseswswnenwswnwnwsewwnwsene\nsewnenenenesenwsewnenwwwse\nwenwwweseeeweswwwnwwe\nwsweesenenewnwwnwsenewsenwwsesesenwne\nneeswseenwwswnwswswnw\nnenwswwsewswnenenewsenwsenwnesesenew\nenewnwewneswsewnwswenweswnenwsenwsw\nsweneswneswneneenwnewenewwneswswnese\nswwesenesewenwneswnwwneseswwne\nenesenwswwswneneswsenwnewswseenwsese\nwnwnesenesenenwwnenwsewesewsesesew\nnenewswnwewswnenesenwnesewesw\neneswnwswnwsenenwnwnwwseeswneewsenese\nneswnwewnwnwseenwseesewsenwsweewe\nwseweeenwnesenwwwswnew";
        let mut game = Game::new(&input);
        game.run_n_days(100);
        assert_eq!(game.count_black_tiles(), 2208);
    }
}
