use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|s| s.lines().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn solve_1(answers: &Vec<Vec<&str>>) -> usize {
    answers.iter().fold(0, |total, answer| {
        total
            + answer
                .iter()
                .fold(HashSet::<char>::new(), |mut set, &s| {
                    s.chars().for_each(|c| {
                        set.insert(c);
                    });
                    set
                })
                .len()
    })
}

fn solve_2(answers: &Vec<Vec<&str>>) -> usize {
    answers.iter().fold(0, |total, answer| {
        total
            + answer[1..]
                .iter()
                .fold(answer[0].chars().collect::<Vec<_>>(), |vec, &s| {
                    vec.iter().filter(|&c| s.contains(*c)).cloned().collect()
                })
                .len()
    })
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day6").unwrap();
    let answers = parse_input(&input);
    println!(
        "day 6 solution 1 : {}, {}us",
        solve_1(&answers),
        timer.elapsed().as_micros()
    );
    println!(
        "day 6 solution 2 : {}, {}us",
        solve_2(&answers),
        timer.elapsed().as_micros()
    );
}
