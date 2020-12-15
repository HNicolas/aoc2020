use std::{fmt::Display, ops::RangeInclusive};

#[derive(Clone, PartialEq)]
enum SeatState {
    Empty,
    Occupied,
    NoSeat,
}

enum Direction {
    Front,
    Back,
    None,
}

const DIRECTIONS: [(Direction, Direction); 8] = [
    (Direction::Back, Direction::Back),
    (Direction::Back, Direction::None),
    (Direction::Back, Direction::Front),
    (Direction::None, Direction::Back),
    (Direction::None, Direction::Front),
    (Direction::Front, Direction::Back),
    (Direction::Front, Direction::None),
    (Direction::Front, Direction::Front),
];

impl Display for SeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            SeatState::Empty => 'L',
            SeatState::Occupied => '#',
            SeatState::NoSeat => '.',
        };
        write!(f, "{}", c)
    }
}

type SeatRow = Vec<SeatState>;
type SeatLayout = Vec<SeatRow>;

#[derive(Clone)]
struct Layout {
    layout: SeatLayout,
}

impl Layout {
    fn new(input: &str) -> Self {
        let mut layout = Vec::with_capacity(input.lines().count());
        for line in input.lines() {
            let mut row = Vec::with_capacity(line.len());
            for c in line.chars() {
                let seat = match c {
                    'L' => SeatState::Empty,
                    '#' => SeatState::Occupied,
                    '.' => SeatState::NoSeat,
                    _ => panic!(format!("invalid char in layout : {}", c)),
                };
                row.push(seat);
            }
            layout.push(row);
        }

        Self { layout }
    }

    fn next_1(&mut self) -> bool {
        let row_count = self.layout.len();
        let column_count = self.layout[0].len();
        let mut new_layout = Vec::with_capacity(row_count);
        let mut mutated = false;
        for i in 0..row_count {
            let mut new_row = Vec::with_capacity(column_count);
            for j in 0..column_count {
                let new_state = match self.layout[i][j] {
                    SeatState::Empty => {
                        let mut state = SeatState::Occupied;
                        let mut seat_mutated = true;
                        'outer_empty: for k in get_safe_range(i, 0, row_count - 1) {
                            for l in get_safe_range(j, 0, column_count - 1) {
                                if k == i && l == j {
                                    continue;
                                }
                                if self.layout[k][l] == SeatState::Occupied {
                                    state = SeatState::Empty;
                                    seat_mutated = false;
                                    break 'outer_empty;
                                }
                            }
                        }
                        mutated = mutated || seat_mutated;
                        state
                    }
                    SeatState::Occupied => {
                        let mut occupied_adjacent = 0;
                        let mut state = SeatState::Occupied;
                        'outer_occupied: for k in get_safe_range(i, 0, row_count - 1) {
                            for l in get_safe_range(j, 0, column_count - 1) {
                                if k == i && l == j {
                                    continue;
                                }
                                if self.layout[k][l] == SeatState::Occupied {
                                    occupied_adjacent += 1;
                                    if occupied_adjacent >= 4 {
                                        state = SeatState::Empty;
                                        mutated = true;
                                        break 'outer_occupied;
                                    }
                                }
                            }
                        }
                        state
                    }
                    SeatState::NoSeat => SeatState::NoSeat,
                };
                new_row.push(new_state);
            }
            new_layout.push(new_row);
        }
        self.layout = new_layout;
        mutated
    }

    fn next_2(&mut self) -> bool {
        let row_count = self.layout.len();
        let column_count = self.layout[0].len();
        let mut new_layout = Vec::with_capacity(row_count);
        let mut mutated = false;
        for i in 0..row_count {
            let mut new_row = Vec::with_capacity(column_count);
            for j in 0..column_count {
                let new_state = match self.layout[i][j] {
                    SeatState::Empty => {
                        let mut state = SeatState::Occupied;
                        let mut seat_mutated = true;

                        for direction in DIRECTIONS.iter() {
                            if self.get_seen_seat((i, j), direction) == SeatState::Occupied {
                                state = SeatState::Empty;
                                seat_mutated = false;
                                break;
                            }
                        }

                        mutated = mutated || seat_mutated;
                        state
                    }
                    SeatState::Occupied => {
                        let mut occupied_adjacent = 0;
                        let mut state = SeatState::Occupied;

                        for direction in DIRECTIONS.iter() {
                            if self.get_seen_seat((i, j), direction) == SeatState::Occupied {
                                occupied_adjacent += 1;
                                if occupied_adjacent >= 5 {
                                    state = SeatState::Empty;
                                    mutated = true;
                                    break;
                                }
                            }
                        }

                        state
                    }
                    SeatState::NoSeat => SeatState::NoSeat,
                };
                new_row.push(new_state);
            }
            new_layout.push(new_row);
        }
        self.layout = new_layout;
        mutated
    }

    fn get_seen_seat(
        &self,
        position: (usize, usize),
        direction: &(Direction, Direction),
    ) -> SeatState {
        let mut i = position.0;
        let mut j = position.1;
        loop {
            match direction.0 {
                Direction::Front => {
                    if i == self.layout.len() - 1 {
                        return SeatState::NoSeat;
                    }
                    i += 1;
                }
                Direction::Back => {
                    if i == 0 {
                        return SeatState::NoSeat;
                    }
                    i -= 1;
                }
                Direction::None => {}
            }
            match direction.1 {
                Direction::Front => {
                    if j == self.layout[0].len() - 1 {
                        return SeatState::NoSeat;
                    }
                    j += 1;
                }
                Direction::Back => {
                    if j == 0 {
                        return SeatState::NoSeat;
                    }
                    j -= 1;
                }
                Direction::None => {}
            }

            match self.layout[i][j] {
                SeatState::NoSeat => {}
                _ => {
                    return self.layout[i][j].clone();
                }
            }
        }
    }

    fn count_occupied(&self) -> usize {
        self.layout.iter().flatten().fold(0, |acc, seat| {
            if *seat == SeatState::Occupied {
                acc + 1
            } else {
                acc
            }
        })
    }

    // fn print(&self) {
    //     for row in &self.layout {
    //         for seat in row {
    //             print!("{}", seat);
    //         }
    //         println!("");
    //     }
    // }
}

fn get_safe_range(value: usize, min: usize, max: usize) -> RangeInclusive<usize> {
    let min = if value > min { value - 1 } else { min };
    let max = if value < max { value + 1 } else { max };
    min..=max
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day11").unwrap();
    let mut layout = Layout::new(&input);
    let mut layout_2 = layout.clone();
    while layout.next_1() {}
    println!(
        "day 11 solution 1 : {}, {}us",
        layout.count_occupied(),
        timer.elapsed().as_micros()
    );
    while layout_2.next_2() {}
    println!(
        "day 11 solution 2 : {}, {}us",
        layout_2.count_occupied(),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut layout = Layout::new(&input);
        while layout.next_1() {}
        assert_eq!(layout.count_occupied(), 37);
    }

    #[test]
    fn test_solution_2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let mut layout = Layout::new(&input);
        while layout.next_2() {}
        assert_eq!(layout.count_occupied(), 26);
    }
}
