use std::collections::{HashMap, HashSet};

fn parse_input(
    input: &str,
) -> (
    HashMap<&str, HashMap<&str, u8>>,
    HashMap<&str, HashMap<&str, u8>>,
) {
    // hold in which bags and how many times a bag is contained
    let mut contained_map: HashMap<&str, HashMap<&str, u8>> = HashMap::new();
    let mut containing_map: HashMap<&str, HashMap<&str, u8>> = HashMap::new();
    for line in input.lines().map(|s| s.trim()) {
        let mut first_split = line.split(" bags contain ");
        let container_color = first_split.next().unwrap();
        let rest = first_split.next().unwrap();
        let mut inner_map = HashMap::new();
        if rest != "no other bags." {
            let mut second_split = rest.split(", ");
            while let Some(content) = second_split.next() {
                let mut third_split = content
                    .trim_end_matches('.')
                    .trim_end_matches('s')
                    .trim_end_matches(" bag")
                    .splitn(2, ' ');
                let count = third_split.next().unwrap().parse().unwrap();
                let bag_color = third_split.next().unwrap();

                if let Some(map) = contained_map.get_mut(bag_color) {
                    map.insert(container_color, count);
                } else {
                    contained_map.insert(
                        bag_color,
                        [(container_color, count)].iter().cloned().collect(),
                    );
                }

                inner_map.insert(bag_color, count);
            }
        }
        containing_map.insert(container_color, inner_map);
    }
    (contained_map, containing_map)
}

fn solve_1(map: &HashMap<&str, HashMap<&str, u8>>) -> usize {
    let mut seen = HashSet::new();
    let mut colors = vec!["shiny gold"];
    while colors.len() > 0 {
        let mut new_colors = vec![];
        for color in colors {
            if let Some(container_map) = map.get(color) {
                for &container_color in container_map.keys() {
                    if !seen.contains(container_color) {
                        seen.insert(container_color);
                        new_colors.push(container_color);
                    }
                }
            }
        }
        colors = new_colors;
    }
    seen.len()
}

fn solve_2(map: &HashMap<&str, HashMap<&str, u8>>, bag_color: &str) -> usize {
    let bag_contents = map.get(bag_color).unwrap();
    if bag_contents.len() > 0 {
        bag_contents.iter().fold(0, |acc, (&key, &value)| {
            acc + value as usize * (solve_2(map, key) + 1)
        })
    } else {
        0
    }
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day7").unwrap();
    let parsed_input = parse_input(&input);
    println!("day 7 solution 1 : {}", solve_1(&parsed_input.0));
    println!("day 7 solution 2 : {}", solve_2(&parsed_input.1, "shiny gold"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parsing() {
        let input = "light blue bags contain no other bags.\ndark green bags contain 2 light blue bags.\nsoft pink bags contain 1 dark green bag, 5 light blue bags.";
        let mut map: HashMap<&str, HashMap<&str, u8>> = HashMap::new();
        map.insert(
            "light blue",
            [("soft pink", 5), ("dark green", 2)]
                .iter()
                .cloned()
                .collect(),
        );
        map.insert("dark green", [("soft pink", 1)].iter().cloned().collect());
        assert_eq!(parse_input(input).0, map);
    }

    #[test]
    fn test_solution_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        assert_eq!(solve_1(&parse_input(input).0), 4);
    }

    #[test]
    fn test_solution_2() {
        let input = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
        assert_eq!(solve_2(&parse_input(input).1, "shiny gold"), 126);
    }
}
