use std::collections::HashMap;

fn solve_1(input: &str) -> u64 {
    let mut memory = HashMap::new();
    let mut or_mask = 0u64;
    let mut and_mask = !0u64;
    for line in input.lines() {
        let mut split = line.split(" = ");
        let instruction = split.next().unwrap();
        match instruction {
            "mask" => {
                for (index, c) in split.next().unwrap().char_indices() {
                    let bit = 1u64 << (35 - index);
                    match c {
                        'X' => {
                            // set the specific bit to 0 for OR and 1 for AND so it is not flipped
                            or_mask &= !bit;
                            and_mask |= bit;
                        }
                        '1' => {
                            // set both to 1
                            or_mask |= bit;
                            and_mask |= bit;
                        }
                        '0' => {
                            // set both to 0
                            or_mask &= !bit;
                            and_mask &= !bit;
                        }
                        _ => panic!("invalid mask char"),
                    }
                }
            }
            _ => {
                // should be mem[xxx]
                let address = instruction[4..instruction.len() - 1]
                    .parse::<u64>()
                    .unwrap();
                let value = split.next().unwrap().parse::<u64>().unwrap();
                memory.insert(address, value & and_mask | or_mask);
            }
        }
    }

    memory.values().fold(0u64, |acc, curr| acc + curr)
}

fn solve_2(input: &str) -> u64 {
    let mut memory = HashMap::new();
    let mut or_mask = 0u64;
    let mut mask = "";
    for line in input.lines() {
        let mut split = line.split(" = ");
        let instruction = split.next().unwrap();
        match instruction {
            "mask" => {
                mask = split.next().unwrap();
                or_mask =
                    mask.char_indices()
                        .filter(|(_, c)| *c == '1')
                        .fold(0u64, |acc, (index, _)| {
                            let bit = 1u64 << (35 - index);
                            acc | bit
                        });
            }
            _ => {
                // should be mem[xxx]
                let address = format!(
                    "{:0>36b}",
                    &instruction[4..instruction.len() - 1]
                        .parse::<u64>()
                        .unwrap()
                );
                let value = split.next().unwrap().parse::<u64>().unwrap();

                let floating_bits = mask
                    .char_indices()
                    .filter(|(_, c)| *c == 'X')
                    .collect::<Vec<_>>();
                if floating_bits.len() > 0 {
                    // generate all the address and write value
                    for i in 0..1u64 << floating_bits.len() {
                        let bin_str = format!("{:0length$b}", i, length = floating_bits.len())
                            .chars()
                            .collect::<Vec<_>>();
                        let mut new_address = String::new();
                        let mut current_index = 0usize;
                        for j in 0..floating_bits.len() {
                            new_address.push_str(&address[current_index..floating_bits[j].0]);
                            new_address.push(bin_str[j]);
                            current_index = floating_bits[j].0 + 1;
                        }
                        new_address.push_str(&address[current_index..]);
                        memory.insert(
                            u64::from_str_radix(&new_address, 2).unwrap() | or_mask,
                            value,
                        );
                    }
                } else {
                    memory.insert(address.parse::<u64>().unwrap() | or_mask, value);
                }
            }
        }
    }

    memory.values().fold(0u64, |acc, curr| acc + curr)
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day14").unwrap();
    println!("day 14 solution 1 : {}", solve_1(&input));
    println!("day 14 solution 2 : {}", solve_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        assert_eq!(solve_1(&input), 165);
    }

    #[test]
    fn test_solution_2() {
        let input =
            "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        assert_eq!(solve_2(&input), 208);
    }
}
