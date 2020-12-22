use std::collections::HashSet;

enum Player {
    One,
    Two,
}

struct Game {
    player_one: Vec<u8>,
    player_two: Vec<u8>,
}

impl Game {
    fn new(input: &str) -> Self {
        let mut players = input.split("\n\n");
        let player_one = players
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        let player_two = players
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        Self {
            player_one,
            player_two,
        }
    }

    fn run(&mut self) {
        while self.player_one.len() > 0 && self.player_two.len() > 0 {
            let one = self.player_one.remove(0);
            let two = self.player_two.remove(0);
            if one > two {
                self.player_one.push(one);
                self.player_one.push(two);
            } else {
                self.player_two.push(two);
                self.player_two.push(one);
            }
        }
    }

    fn run_recursive(&mut self) -> Player {
        let mut previous_configurations = HashSet::new();
        while !previous_configurations.contains(&self.player_one)
            && self.player_one.len() > 0
            && self.player_two.len() > 0
        {
            previous_configurations.insert(self.player_one.clone());

            let one = self.player_one.remove(0);
            let two = self.player_two.remove(0);
            let winner =
                if one as usize <= self.player_one.len() && two as usize <= self.player_two.len() {
                    let mut game = Game {
                        player_one: self.player_one[..one as usize].iter().copied().collect(),
                        player_two: self.player_two[..two as usize].iter().copied().collect(),
                    };
                    game.run_recursive()
                } else if one > two {
                    Player::One
                } else {
                    Player::Two
                };
            match winner {
                Player::One => {
                    self.player_one.push(one);
                    self.player_one.push(two);
                }
                Player::Two => {
                    self.player_two.push(two);
                    self.player_two.push(one);
                }
            }
        }

        if self.player_one.len() == 0 {
            Player::Two
        } else {
            Player::One
        }
    }
}

fn solve_1(game: &mut Game) -> u32 {
    game.run();
    let winner = if game.player_one.len() > 0 {
        &game.player_one
    } else {
        &game.player_two
    };
    winner.iter().enumerate().fold(0, |acc, (index, &value)| {
        acc + value as u32 * (winner.len() - index) as u32
    })
}

fn solve_2(game: &mut Game) -> u32 {
    let winner = match game.run_recursive() {
        Player::One => &game.player_one,
        Player::Two => &game.player_two,
    };
    winner.iter().enumerate().fold(0, |acc, (index, &value)| {
        acc + value as u32 * (winner.len() - index) as u32
    })
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day22").unwrap();
    let mut game = Game::new(&input);
    let mut game2 = Game {
        player_one: game.player_one.clone(),
        player_two: game.player_two.clone(),
    };
    println!(
        "day 22 solution 1 : {}, {}us",
        solve_1(&mut game),
        timer.elapsed().as_micros()
    );
    println!(
        "day 22 solution 2 : {}, {}us",
        solve_2(&mut game2),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solution_1() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        let mut game = Game::new(&input);
        assert_eq!(solve_1(&mut game), 306);
    }

    #[test]
    fn test_solution_2() {
        let input = "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10";
        let mut game = Game::new(&input);
        assert_eq!(solve_2(&mut game), 291);
    }
}
