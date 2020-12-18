#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn parse(input: &str) -> (Self, &str) {
        match input.chars().next() {
            Some('*') => (Self::Mul, &input[2..]),
            Some('+') => (Self::Add, &input[2..]),
            _ => panic!(format!("invalid operator {}", input)),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Number(u64),
    Operation(Box<Operation>),
}

impl Operand {
    fn parse(input: &str, use_precedence: bool) -> (Self, Option<&str>) {
        let mut chars = input.char_indices();
        match chars.next() {
            Some((_, '(')) => {
                // find matching parenthesis and parse operation from str
                let mut deepness = 0;
                for (index, c) in chars {
                    match c {
                        '(' => {
                            deepness += 1;
                        }
                        ')' if deepness == 0 => {
                            let rest = if index == input.len() - 1 {
                                None
                            } else {
                                Some(&input[index + 2..])
                            };
                            return (
                                Self::Operation(Box::new(Operation::parse(&input[1..index], use_precedence))),
                                rest,
                            );
                        }
                        ')' if deepness > 0 => {
                            deepness -= 1;
                        }
                        _ => {}
                    }
                }
                panic!("invalid operation");
            }
            _ => {
                // extract number
                let mut split = input.splitn(2, ' ');
                let operand = split.next().unwrap().parse::<u64>().unwrap();
                let rest = split.next();
                (Self::Number(operand), rest)
            }
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    lhs: Operand,
    rhs: Operand,
}

impl Operation {
    fn parse(input: &str, use_precedence: bool) -> Self {
        let (lhs, r) = Operand::parse(input, use_precedence);
        let (operator, r) = Operator::parse(r.unwrap());
        let (rhs, r) = Operand::parse(r, use_precedence);

        let mut operation = Self { lhs, operator, rhs };
        let mut rest = r;
        while let Some(r) = rest {
            if r.len() == 0 {
                break;
            }
            let (operator, r) = Operator::parse(r);
            let (rhs, r) = Operand::parse(r, use_precedence);

            if use_precedence && operator == Operator::Add && operation.operator == Operator::Mul {
                operation.rhs = Operand::Operation(Box::new(Self {
                    lhs: operation.rhs,
                    operator,
                    rhs,
                }));
            } else {
                operation = Self {
                    lhs: Operand::Operation(Box::new(operation)),
                    operator,
                    rhs,
                };
            }

            rest = r;
        }
        operation
    }

    fn solve(&self) -> u64 {
        let lhs = match &self.lhs {
            &Operand::Number(n) => n,
            Operand::Operation(o) => o.solve(),
        };
        let rhs = match &self.rhs {
            &Operand::Number(n) => n,
            Operand::Operation(o) => o.solve(),
        };
        match &self.operator {
            Operator::Mul => lhs * rhs,
            Operator::Add => lhs + rhs,
        }
    }
}

fn solve(input: &str, use_precedence: bool) -> u64 {
    input
        .lines()
        .fold(0u64, |acc, curr| acc + Operation::parse(curr, use_precedence).solve())
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day18").unwrap();
    println!(
        "day 18 solution 1 : {}, {}us",
        solve(&input, false),
        timer.elapsed().as_micros()
    );
    println!(
        "day 18 solution 2 : {}, {}us",
        solve(&input, true),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "2 * 3 + (4 * 5)";
        let operation = Operation::parse(input, false);
        assert_eq!(operation.solve(), 26);
    }

    #[test]
    fn test_2() {
        let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let operation = Operation::parse(input, false);
        assert_eq!(operation.solve(), 13632);
    }

    #[test]
    fn test_3() {
        let input = "2 * 3 + (4 * 5)";
        let operation = Operation::parse(input, true);
        assert_eq!(operation.solve(), 46);
    }
}
