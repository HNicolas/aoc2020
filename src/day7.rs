use std::collections::{HashMap, HashSet};

struct Bag<'a> {
    parents: HashSet<&'a str>,
    children: HashMap<&'a str, u8>,
}

impl<'a> Bag<'a> {
    fn new() -> Self {
        Bag {
            parents: HashSet::new(),
            children: HashMap::new(),
        }
    }
}

fn parse_input(input: &str) -> HashMap<&str, Bag> {
    let mut bag_map: HashMap<&str, Bag> = HashMap::new();
    for line in input.lines().map(|s| s.trim()) {
        let mut first_split = line.split(" bags contain ");
        let bag_color = first_split.next().unwrap();
        let rest = first_split.next().unwrap();

        if !bag_map.contains_key(bag_color) {
            bag_map.insert(bag_color, Bag::new());
        }

        if rest != "no other bags." {
            let mut second_split = rest.split(", ");
            while let Some(content) = second_split.next() {
                let mut third_split = content
                    .trim_end_matches('.')
                    .trim_end_matches('s')
                    .trim_end_matches(" bag")
                    .splitn(2, ' ');
                let count = third_split.next().unwrap().parse().unwrap();
                let inner_bag_color = third_split.next().unwrap();

                {
                    let bag = bag_map.get_mut(bag_color).unwrap();
                    bag.children.insert(inner_bag_color, count);
                }

                if let Some(inner_bag) = bag_map.get_mut(inner_bag_color) {
                    inner_bag.parents.insert(bag_color);
                } else {
                    let mut inner_bag = Bag::new();
                    inner_bag.parents.insert(bag_color);
                    bag_map.insert(inner_bag_color, inner_bag);
                }
            }
        }
    }
    bag_map
}

fn solve_1(bag_map: &HashMap<&str, Bag>) -> usize {
    let mut seen = HashSet::new();
    let mut colors = vec!["shiny gold"];
    while colors.len() > 0 {
        let mut new_colors = vec![];
        for color in colors {
            if let Some(bag) = bag_map.get(color) {
                for &container_color in &bag.parents {
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

fn solve_2(bag_map: &HashMap<&str, Bag>, bag_color: &str) -> usize {
    let bag_contents = bag_map.get(bag_color).unwrap();
    if bag_contents.children.len() > 0 {
        bag_contents.children.iter().fold(0, |acc, (&key, &value)| {
            acc + value as usize * (solve_2(bag_map, key) + 1)
        })
    } else {
        0
    }
}

pub fn run() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("inputs/day7").unwrap();
    let parsed_input = parse_input(&input);
    println!(
        "day 7 solution 1 : {}, {}us",
        solve_1(&parsed_input),
        timer.elapsed().as_micros()
    );
    println!(
        "day 7 solution 2 : {}, {}us",
        solve_2(&parsed_input, "shiny gold"),
        timer.elapsed().as_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        assert_eq!(solve_1(&parse_input(input)), 4);
    }

    #[test]
    fn test_solution_2() {
        let input = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
        assert_eq!(solve_2(&parse_input(input), "shiny gold"), 126);
    }
}
