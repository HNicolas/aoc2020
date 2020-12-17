use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates(i16, i16, i16);

impl Coordinates {
    fn get_neighboors(&self) -> Vec<Self> {
        (self.0 - 1..=self.0 + 1)
            .flat_map(|x| {
                (self.1 - 1..=self.1 + 1).flat_map(move |y| {
                    (self.2 - 1..=self.2 + 1).filter_map(move |z| {
                        if Self(x, y, z) == *self {
                            None
                        } else {
                            Some(Self(x, y, z))
                        }
                    })
                })
            })
            .collect::<Vec<_>>()
    }
}

struct Game {
    active_cells: HashSet<Coordinates>,
}

impl Game {
    fn new(input: &str) -> Self {
        let active_cells = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars().enumerate().filter_map(move |(y, c)| {
                    if c == '#' {
                        Some(Coordinates(x as i16, y as i16, 0))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self { active_cells }
    }

    fn next_state(&mut self) {
        let mut active_neighbours_map = HashMap::<Coordinates, u8>::new();
        for active_cell in &self.active_cells {
            for neighbour_cell in active_cell.get_neighboors() {
                if let Some(active_neighbours) = active_neighbours_map.get_mut(&neighbour_cell) {
                    *active_neighbours += 1;
                } else {
                    active_neighbours_map.insert(neighbour_cell, 1);
                }
            }
        }

        let mut new_active_cells = HashSet::new();
        for (cell, active_neighboors) in active_neighbours_map {
            match active_neighboors {
                3 => {
                    new_active_cells.insert(cell);
                }
                2 if self.active_cells.contains(&cell) => {
                    new_active_cells.insert(cell);
                }
                _ => {}
            }
        }
        self.active_cells = new_active_cells;
    }

    fn compute_cycles(&mut self, number: usize) {
        for _ in 0..number {
            self.next_state();
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates4d(i16, i16, i16, i16);

impl Coordinates4d {
    fn get_neighboors(&self) -> Vec<Self> {
        (self.0 - 1..=self.0 + 1)
            .flat_map(|x| {
                (self.1 - 1..=self.1 + 1).flat_map(move |y| {
                    (self.2 - 1..=self.2 + 1).flat_map(move |z| {
                        (self.3 - 1..=self.3 + 1).filter_map(move |w| {
                            if Self(x, y, z, w) == *self {
                                None
                            } else {
                                Some(Self(x, y, z, w))
                            }
                        })
                    })
                })
            })
            .collect::<Vec<_>>()
    }
}

struct Game4d {
    active_cells: HashSet<Coordinates4d>,
}

impl Game4d {
    fn new(input: &str) -> Self {
        let active_cells = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars().enumerate().filter_map(move |(y, c)| {
                    if c == '#' {
                        Some(Coordinates4d(x as i16, y as i16, 0, 0))
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self { active_cells }
    }

    fn next_state(&mut self) {
        let mut active_neighbours_map = HashMap::<Coordinates4d, u8>::new();
        for active_cell in &self.active_cells {
            for neighbour_cell in active_cell.get_neighboors() {
                if let Some(active_neighbours) = active_neighbours_map.get_mut(&neighbour_cell) {
                    *active_neighbours += 1;
                } else {
                    active_neighbours_map.insert(neighbour_cell, 1);
                }
            }
        }

        let mut new_active_cells = HashSet::new();
        for (cell, active_neighboors) in active_neighbours_map {
            match active_neighboors {
                3 => {
                    new_active_cells.insert(cell);
                }
                2 if self.active_cells.contains(&cell) => {
                    new_active_cells.insert(cell);
                }
                _ => {}
            }
        }
        self.active_cells = new_active_cells;
    }

    fn compute_cycles(&mut self, number: usize) {
        for _ in 0..number {
            self.next_state();
        }
    }
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day17").unwrap();
    let mut game = Game::new(&input);
    game.compute_cycles(6);
    println!(
        "day 17 solution 1 : {}, {}us",
        game.active_cells.len(),
        timer.elapsed().as_micros()
    );

    let mut game = Game4d::new(&input);
    game.compute_cycles(6);
    println!(
        "day 17 solution 2 : {}, {}us",
        game.active_cells.len(),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = ".#.\n..#\n###";
        let mut game = Game::new(&input);
        game.compute_cycles(6);
        assert_eq!(game.active_cells.len(), 112);
    }

    #[test]
    fn test_solution_2() {
        let input = ".#.\n..#\n###";
        let mut game = Game4d::new(&input);
        game.compute_cycles(6);
        assert_eq!(game.active_cells.len(), 848);
    }
}
