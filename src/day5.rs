use std::collections::HashSet;

fn parse_input(input: &str) -> HashSet<u16> {
    input
        .lines()
        .map(|s| {
            s.chars()
                .fold(String::with_capacity(s.len()), |mut acc, c| {
                    match c {
                        'F' | 'L' => {
                            acc.push('0');
                        }
                        'B' | 'R' => {
                            acc.push('1');
                        }
                        _ => {}
                    }
                    acc
                })
        })
        .map(|s| u16::from_str_radix(&s, 2).unwrap())
        .collect()
}

fn solve_1(numbers: &HashSet<u16>) -> u16 {
    *numbers.iter().max().unwrap()
}

fn solve_2(numbers: &HashSet<u16>, max: u16) -> u16 {
    let mut i = max;
    while numbers.contains(&i) {
        i -= 1;
    }
    i
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day5").unwrap();
    let numbers = parse_input(&input);
    let max = solve_1(&numbers);
    println!("day 5 solution 1 : {}, {}us", max, timer.elapsed().as_micros());
    println!("day 5 solution 2 : {}, {}us", solve_2(&numbers, max), timer.elapsed().as_micros());
}
