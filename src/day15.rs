use std::collections::HashMap;

fn solve(input: &str, nth: usize) -> u32 {
    let numbers = input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut map = numbers[0..numbers.len() - 1]
        .iter()
        .enumerate()
        .map(|(a, b)| (*b, a))
        .collect::<HashMap<_, _>>();
    let mut last_value = *numbers.last().unwrap();
    for i in numbers.len()..nth {
        let next_value = if map.contains_key(&last_value) {
            (i - 1 - map.get(&last_value).unwrap()) as u32
        }else {
            0
        };
        map.insert(last_value, i - 1);
        last_value = next_value;
    }
    last_value
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day15").unwrap();
    println!("day 15 solution 1 : {}", solve(&input, 2020));
    println!("day 15 solution 2 : {}", solve(&input, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "3,1,2";
        assert_eq!(solve(&input, 2020), 1836);
    }
}
