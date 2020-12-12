#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Action {
    fn new(action: &str) -> Self {
        match &action[..1] {
            "N" => Action::North(action[1..].parse().unwrap()),
            "S" => Action::South(action[1..].parse().unwrap()),
            "E" => Action::East(action[1..].parse().unwrap()),
            "W" => Action::West(action[1..].parse().unwrap()),
            "L" => Action::Left(action[1..].parse().unwrap()),
            "R" => Action::Right(action[1..].parse().unwrap()),
            "F" => Action::Forward(action[1..].parse().unwrap()),
            _ => panic!("invalid action"),
        }
    }
}

struct Instruction {
    actions: Vec<Action>,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let mut actions = vec![];
        for line in input.lines() {
            actions.push(Action::new(line));
        }
        Self { actions }
    }

    fn solve_1(&self) -> u32 {
        let mut angle = 0;
        let mut north = 0;
        let mut east = 0;
        for action in &self.actions {
            match action {
                Action::North(value) => {
                    north += value;
                }
                Action::South(value) => {
                    north -= value;
                }
                Action::East(value) => {
                    east += value;
                }
                Action::West(value) => {
                    east -= value;
                }
                Action::Left(value) => {
                    angle += value;
                }
                Action::Right(value) => {
                    angle -= value;
                }
                Action::Forward(value) => {
                    north += value * (angle as f32).to_radians().sin() as i32;
                    east += value * (angle as f32).to_radians().cos() as i32;
                }
            }
        }
        (north.abs() + east.abs()) as u32
    }

    fn solve_2(&self) -> u32 {
        let mut waypoint = (10, 1);
        let mut ship = (0, 0);
        for action in &self.actions {
            match action {
                Action::North(value) => {
                    waypoint.1 += value;
                }
                Action::South(value) => {
                    waypoint.1 -= value;
                }
                Action::East(value) => {
                    waypoint.0 += value;
                }
                Action::West(value) => {
                    waypoint.0 -= value;
                }
                Action::Left(theta) => {
                    let theta = (*theta as f32).to_radians();
                    waypoint = rotate(waypoint, theta);
                }
                Action::Right(theta) => {
                    let theta = -(*theta as f32).to_radians();
                    waypoint = rotate(waypoint, theta);
                }
                Action::Forward(value) => {
                    ship.0 += waypoint.0 * value;
                    ship.1 += waypoint.1 * value;
                }
            }
        }
        (ship.0.abs() + ship.1.abs()) as u32
    }
}

fn rotate(coordinates: (i32, i32), theta: f32) -> (i32, i32) {
    (
        coordinates.0 * theta.cos() as i32 - coordinates.1 * theta.sin() as i32,
        coordinates.0 * theta.sin() as i32 + coordinates.1 * theta.cos() as i32,
    )
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day12").unwrap();
    let instruction = Instruction::new(&input);
    println!("day 12 solution 1 : {}", instruction.solve_1());
    println!("day 12 solution 2 : {}", instruction.solve_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let instruction = Instruction::new(&input);
        assert_eq!(instruction.solve_1(), 25);
    }

    #[test]
    fn test_solution_2() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let instruction = Instruction::new(&input);
        assert_eq!(instruction.solve_2(), 286);
    }
}
