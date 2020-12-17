use std::collections::{HashMap, HashSet};

struct CellNeighbours<'a> {
    cell: &'a Vec<i16>,
    index: u8,
    total: u8,
}

impl<'a> CellNeighbours<'a> {
    fn new(cell: &'a Vec<i16>) -> Self {
        Self {
            cell,
            index: 0,
            total: 3u8.pow(cell.len() as u32),
        }
    }
}

impl<'a> Iterator for CellNeighbours<'a> {
    type Item = Vec<i16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == (self.total - 1) / 2 {
            self.index += 1;
        }

        if self.index == self.total {
            None
        } else {
            let sum_vec = to_base(self.index, 3);
            let result = self
                .cell
                .iter()
                .enumerate()
                .fold(vec![], |mut vec, (index, value)| {
                    vec.push(
                        value - 1
                            + if index < sum_vec.len() {
                                sum_vec[index] as i16
                            } else {
                                0
                            },
                    );
                    vec
                });
            self.index += 1;
            Some(result)
        }
    }
}

fn to_base(number: u8, base: u8) -> Vec<u8> {
    let mut result = vec![];
    let mut quot = number;
    loop {
        let rest = quot % base;
        result.push(rest);
        quot = (quot - rest) / base;
        if quot == 0 {
            break result;
        }
    }
}

struct Game {
    active_cells: HashSet<Vec<i16>>,
}

impl Game {
    fn new(input: &str, dimension: usize) -> Self {
        let active_cells = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars().enumerate().filter_map(move |(y, c)| {
                    if c == '#' {
                        let mut vec = vec![0; dimension];
                        vec[0] = x as i16;
                        vec[1] = y as i16;
                        Some(vec)
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self { active_cells }
    }

    fn next_state(&mut self) {
        let mut active_neighbours_map = HashMap::<Vec<i16>, u8>::new();
        for active_cell in &self.active_cells {
            for neighbour_cell in CellNeighbours::new(active_cell) {
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
    let mut game = Game::new(&input, 3);
    game.compute_cycles(6);
    println!(
        "day 17 solution 1 : {}, {}us",
        game.active_cells.len(),
        timer.elapsed().as_micros()
    );

    let mut game = Game::new(&input, 4);
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
        let mut game = Game::new(&input, 3);
        game.compute_cycles(6);
        assert_eq!(game.active_cells.len(), 112);
    }

    #[test]
    fn test_solution_2() {
        let input = ".#.\n..#\n###";
        let mut game = Game::new(&input, 4);
        game.compute_cycles(6);
        assert_eq!(game.active_cells.len(), 848);
    }
}
