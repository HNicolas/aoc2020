fn solve_1(input: &str) -> String {
    let numbers = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let map = solve(&numbers, 100);
    let mut result = String::with_capacity(8);
    let mut current_number = 1;
    while result.len() < 8 {
        current_number = map[current_number as usize - 1];
        result.push_str(&current_number.to_string());
    }
    result
}

fn solve_2(input: &str) -> u128 {
    let mut numbers = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    numbers.extend(10..=1000000);
    let map = solve(&numbers, 10000000);
    let one_next = map[0];
    let one_next_next = map[one_next as usize - 1];
    one_next as u128 * one_next_next as u128
}

fn solve(numbers: &Vec<u32>, iterations: usize) -> Vec<u32> {
    let mut map = vec![0; numbers.len()];
    for i in 0..numbers.len() {
        map[numbers[i] as usize - 1] = numbers[(i + 1) % numbers.len()];
    }

    let mut current_number = numbers[0];
    for _ in 0..iterations {
        let mut destination =
            (current_number + numbers.len() as u32 - 2) % numbers.len() as u32 + 1;

        let first_following = map[current_number as usize - 1];
        let second_following = map[first_following as usize - 1];
        let third_following = map[second_following as usize - 1];
        while destination == first_following
            || destination == second_following
            || destination == third_following
        {
            destination = (destination + numbers.len() as u32 - 2) % numbers.len() as u32 + 1;
        }

        // update current next to be next of the third
        map[current_number as usize - 1] = map[third_following as usize - 1];
        // update third next to be next of destination
        map[third_following as usize - 1] = map[destination as usize - 1];
        // update destination next to be next of current
        map[destination as usize - 1] = first_following;

        current_number = map[current_number as usize - 1];
    }

    map
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = "253149867";
    println!(
        "day 23 solution 1 : {}, {}us",
        solve_1(&input),
        timer.elapsed().as_micros()
    );
    println!(
        "day 23 solution 2 : {}, {}us",
        solve_2(&input),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "389125467";
        assert_eq!(solve_1(&input), "67384529");
    }

    #[test]
    fn test_solution_2() {
        let input = "389125467";
        assert_eq!(solve_2(&input), 149245887792);
    }
}
