use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn new(instruction: &str) -> Self {
        let mut split = instruction.split(' ');
        let operation = split.next().unwrap();
        let argument = split.next().unwrap().parse().unwrap();
        match operation {
            "acc" => Instruction::Acc(argument),
            "jmp" => Instruction::Jmp(argument),
            "nop" => Instruction::Nop(argument),
            _ => panic!("invalid operation"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

fn run_instructions(instructions: &Vec<Instruction>, swap_index: Option<usize>) -> (i32, usize) {
    let mut acc = 0;
    let mut index = 0;
    let mut seen = HashSet::new();

    while !seen.contains(&index) && index < instructions.len() {
        seen.insert(index);
        match instructions[index] {
            Instruction::Acc(arg) => {
                acc += arg;
                index += 1;
            }
            Instruction::Jmp(arg) => {
                if swap_index == Some(index) {
                    index += 1;
                } else {
                    index = (index as i32 + arg) as usize;
                }
            }
            Instruction::Nop(arg) => {
                if swap_index == Some(index) {
                    index = (index as i32 + arg) as usize;
                } else {
                    index += 1;
                }
            }
        }
    }
    (acc, index)
}

fn solve_1(instructions: &Vec<Instruction>) -> i32 {
    run_instructions(instructions, None).0
}

fn solve_2(instructions: &Vec<Instruction>) -> i32 {
    for swap_index in 0..instructions.len() {
        if let Instruction::Acc(_) = instructions[swap_index] {
            continue;
        }

        let (acc, index) = run_instructions(instructions, Some(swap_index));

        if index == instructions.len() {
            return acc;
        }
    }
    panic!("solution not found");
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day8").unwrap();
    let instructions = parse_input(&input);
    println!("day 8 solution 1 : {}", solve_1(&instructions));
    println!("day 8 solution 2 : {}", solve_2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parsing() {
        assert_eq!(Instruction::new("acc -3"), Instruction::Acc(-3));
        assert_eq!(Instruction::new("jmp 3"), Instruction::Jmp(3));
        assert_eq!(Instruction::new("nop 12"), Instruction::Nop(12));
    }

    #[test]
    fn test_solve_1() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        assert_eq!(solve_1(&parse_input(input)), 5);
    }

    #[test]
    fn test_solve_2() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        assert_eq!(solve_2(&parse_input(input)), 8);
    }
}
