use std::collections::HashMap;

fn solve(input: &str, nth: usize) -> u32 {
    let numbers = input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut map = numbers[0..numbers.len() - 1]
        .iter()
        .enumerate()
        .map(|(index, number)| (*number, index + 1))
        .collect::<HashMap<_, _>>();
    let mut last_spoken = *numbers.last().unwrap();
    for index in numbers.len()..nth {
        last_spoken = if let Some(last_index) = map.insert(last_spoken, index) {
            (index - last_index) as u32
        } else {
            0
        };
    }
    last_spoken
}

fn solve_test(input: &str, nth: usize) -> u32 {
    let input = input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut numbers: Vec<Option<u32>> = vec![None; nth];
    for index in 1..input.len() {
        numbers[input[index - 1] as usize] = Some(index as u32);
    }
    let mut last_spoken = *input.last().unwrap();
    for index in input.len()..nth {
        let spoken = if let Some(last_index) = numbers[last_spoken as usize] {
            index as u32 - last_index
        } else {
            0
        };
        numbers[last_spoken as usize] = Some(index as u32);
        last_spoken = spoken;
    }
    last_spoken
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day15").unwrap();
    println!(
        "day 15 solution 1 : {}, {}us",
        solve_test(&input, 2020),
        timer.elapsed().as_micros()
    );
    println!(
        "day 15 solution 2 : {}, {}us",
        solve_test(&input, 30000000),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "3,1,2";
        assert_eq!(solve(&input, 2020), 1836);
    }

    #[test]
    fn test_solution_test() {
        let input = "3,1,2";
        assert_eq!(solve_test(&input, 2020), 1836);
    }
}
