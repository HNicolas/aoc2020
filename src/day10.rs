use std::collections::HashMap;

fn solve_1(values: &Vec<u8>) -> u32 {
    let mut current_jolt = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 0;

    loop {
        if values.contains(&(current_jolt + 1)) {
            current_jolt += 1;
            diff_1 += 1;
            continue;
        }
        if values.contains(&(current_jolt + 2)) {
            current_jolt += 2;
            continue;
        }
        if values.contains(&(current_jolt + 3)) {
            current_jolt += 3;
            diff_3 += 1;
            continue;
        }
        break;
    }
    diff_1 * (diff_3 + 1)
}

/*
Avoid recursion
1 -> from 0 -> 1
4 -> from 1 -> 1
5 -> from 4 -> 1
6 -> from 4 or 5 -> 1 + 1 = 2
7 -> from 4 or 5 or 6 -> 1 + 1 + 2 = 4
10 -> from 7 -> 4
11 -> from 10 -> 4
12 -> from 10 or 11 -> 4 + 4 = 8
15 -> from 12 -> 8
16 -> from 15 -> 8
19 -> from 16 -> 8
*/
fn solve_2(values: &mut Vec<u8>) -> u64 {
    values.sort();

    let mut map: HashMap<u8, u64> = HashMap::new();
    map.insert(0, 1);

    for &value in values.iter() {
        let mut paths = 0;
        if value > 0 && map.contains_key(&(value - 1)) {
            paths += map.get(&(value - 1)).unwrap();
        }
        if value > 1 && map.contains_key(&(value - 2)) {
            paths += map.get(&(value - 2)).unwrap();
        }
        if value > 2 && map.contains_key(&(value - 3)) {
            paths += map.get(&(value - 3)).unwrap();
        }
        map.insert(value, paths);
    }

    map.get(&values[values.len() - 1]).unwrap().to_owned()
}

fn parse_input(input: &str) -> Vec<u8> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day10").unwrap();
    let mut values = parse_input(&input);
    println!("day 10 solution 1 : {}", solve_1(&values));
    println!("day 10 solution 2 : {}", solve_2(&mut values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        let values = parse_input(&input);
        assert_eq!(solve_1(&values), 220);
    }

    #[test]
    fn test_solution_2() {
        let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
        let mut values = parse_input(&input);
        assert_eq!(solve_2(&mut values), 8);
    }

    #[test]
    fn test_solution_2_2() {
        let input = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";
        let mut values = parse_input(&input);
        assert_eq!(solve_2(&mut values), 19208);
    }
}
