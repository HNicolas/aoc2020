pub fn run() {
    let timer = std::time::Instant::now();
    let contents = std::fs::read_to_string("inputs/day1").unwrap();
    let numbers = contents.lines().map(|s| s.parse::<u32>().unwrap());

    let mut first_set = std::collections::HashSet::new();
    let mut second_map = std::collections::HashMap::new();
    let mut seen_numbers = vec![];
    let mut first_found = false;
    let mut second_found = false;
    for number in numbers {
        let comp = 2020 - number;
        if first_set.contains(&comp) {
            println!(
                "day 1 solution 1 : {}, {}us",
                number * comp,
                timer.elapsed().as_micros()
            );
            first_found = true;
        }

        if second_map.contains_key(&comp) {
            println!(
                "day 1 solution 2 : {}, {}us",
                number * second_map.get(&comp).unwrap(),
                timer.elapsed().as_micros()
            );
            second_found = true;
        }

        first_set.insert(number);
        for seen_number in &seen_numbers {
            second_map.insert(seen_number + number, seen_number * number);
        }
        seen_numbers.push(number);

        if first_found && second_found {
            break;
        }
    }
}
