#[derive(Debug, PartialEq)]
struct Policy {
    first: u8,
    second: u8,
    character: char,
    password: String,
}

impl Policy {
    fn new(policy: &str) -> Self {
        let mut first_split = policy.splitn(2, '-');
        let first = first_split.next().unwrap().parse().unwrap();
        let mut second_split = first_split.next().unwrap().splitn(2, ' ');
        let second = second_split.next().unwrap().parse().unwrap();
        let mut third_split = second_split.next().unwrap().splitn(2, ": ");
        let character = third_split.next().unwrap().parse().unwrap();
        let password = third_split.next().unwrap().to_owned();
        Policy {
            first,
            second,
            character,
            password,
        }
    }

    fn is_valid_1(&self) -> bool {
        let char_count = self
            .password
            .chars()
            .filter(|&c| c == self.character)
            .count();
        char_count >= self.first as usize && char_count <= self.second as usize
    }

    fn is_valid_2(&self) -> bool {
        self.password.chars().nth(self.first as usize - 1).unwrap() == self.character
            && self.password.chars().nth(self.second as usize - 1).unwrap() == self.character
    }
}

fn parse_input(input: &str) -> Vec<Policy> {
    input.lines().fold(vec![], |mut acc, line| {
        acc.push(Policy::new(line));
        acc
    })
}

fn solve_1(policies: &Vec<Policy>) -> u32 {
    policies.iter().fold(
        0,
        |acc, policy| if policy.is_valid_1() { acc + 1 } else { acc },
    )
}

fn solve_2(policies: &Vec<Policy>) -> u32 {
    policies.iter().fold(
        0,
        |acc, policy| if policy.is_valid_2() { acc + 1 } else { acc },
    )
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day2").unwrap();
    let policies = parse_input(&input);
    println!("day 2 solution 1 : {}", solve_1(&policies));
    println!("day 2 solution 2 : {}", solve_2(&policies));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("1-3 a: abcde"),
            vec![Policy {
                first: 1,
                second: 3,
                character: 'a',
                password: "abcde".to_owned()
            }]
        )
    }

    #[test]
    fn test_policy_validity_1() {
        assert!(Policy::new("1-3 a: abcde").is_valid_1());
    }

    #[test]
    fn test_solve_1() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(solve_1(&parse_input(input)), 2);
    }

    #[test]
    fn test_policy_validity_2() {
        assert!(!Policy::new("1-3 a: abcde").is_valid_2());
    }

    #[test]
    fn test_solve_2() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(solve_2(&parse_input(input)), 1);
    }
}
