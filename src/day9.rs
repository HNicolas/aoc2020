use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

fn solve_1(numbers: &Vec<i64>, preamble_size: usize) -> i64 {
    let mut index = preamble_size;

    let mut set = HashSet::new();
    numbers[..index].iter().for_each(|&number| {
        set.insert(number);
    });

    while index < numbers.len() {
        let mut found = false;
        for &available in &set {
            let comp = numbers[index] - available;
            if comp != available && set.contains(&comp) {
                found = true;
                break;
            }
        }
        if !found {
            return numbers[index];
        }

        set.remove(&numbers[index - preamble_size]);
        set.insert(numbers[index]);
        index += 1;
    }
    panic!("no number found");
}

fn solve_2(numbers: &Vec<i64>, invalid_number: i64) -> i64 {
    let mut start = 0;
    let mut end = 1;
    let mut sum = numbers[start] + numbers[end];
    while sum != invalid_number {
        if sum > invalid_number {
            sum -= numbers[start];
            start += 1;
            if start == end {
                end += 1;
                sum += numbers[end];
            }
        } else {
            end += 1;
            sum += numbers[end];
        }
    }
    let min = numbers[start..=end].iter().min().unwrap();
    let max = numbers[start..=end].iter().max().unwrap();
    min + max
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day9").unwrap();
    let numbers = parse_input(&input);
    let invalid_number = solve_1(&numbers, 25);
    println!(
        "day 9 solution 1 : {}, {}us",
        invalid_number,
        timer.elapsed().as_micros()
    );
    println!(
        "day 9 solution 2 : {}, {}us",
        solve_2(&numbers, invalid_number),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_numbers() -> Vec<i64> {
        let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";
        parse_input(&input)
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(solve_1(&get_numbers(), 5), 127);
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(solve_2(&get_numbers(), 127), 62);
    }
}
